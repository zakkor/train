use rand::Rng;
use std::thread::JoinHandle;
use std::thread;
use std::collections::VecDeque;
use std::sync::mpsc;

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;
use sfml::audio::*;

use state_stack::*;
use resource_manager::*;
use particle_manager::*;
use actor::{Actor};
use menu::*;
use wagon::*;
use game_consts::*;
use world::*;
use camera::*;
use enemy::*;
use pathfinding::*;
use train::*;
use std::sync::mpsc::*;

pub struct Game<'a> {
    resources: &'a Resources,
    music_manager: &'a mut MusicManager,
    window: RenderWindow,
    state_stack: StateStack,
    pm: ParticleManager<'a>,
    clock: Clock,
    pf_clock: Clock,
    train: Train<'a>,
    actors: Vec<Actor<'a>>,
    enemies: Vec<Enemy<'a>>,
    selected_actors: Vec<usize>,
    menu: Menu<'a>,
    world: World<'a>,
    camera: Camera,
    tile_selection: RectangleShape<'a>,
    paused_text: Text<'a>,
    is_paused: bool,
    selection_rect: RectangleShape<'a>,
    selecting: bool,
    handles: Vec<(usize, JoinHandle<Option<VecDeque<(i32, i32)>>>)>,
    channel: (Sender<usize>, Receiver<usize>),
}

impl<'a> Game<'a> {
    pub fn new(resources: &'a Resources, music_manager: &'a mut MusicManager) -> Self {
        // Create the window of the application
        let mut window = RenderWindow::new(VideoMode::new_init(WINDOW_SIZE_X, WINDOW_SIZE_Y, 32),
                                           "Train",
                                           window_style::CLOSE,
                                           &ContextSettings::default())
            .unwrap();

        window.set_framerate_limit(120);
        window.set_vertical_sync_enabled(true);

        let mut state_stack = StateStack::new();
        state_stack.push(StateType::Playing);

        let mut tile_selection = RectangleShape::new().unwrap();
        tile_selection.set_size2f(TILE_SIZE_X as f32, TILE_SIZE_Y as f32);
        tile_selection.set_fill_color(&Color::new_rgba(255, 255, 0, 60));

        let mut selection_rect = RectangleShape::new().unwrap();
        selection_rect.set_size2f(0., 0.);
        selection_rect.set_fill_color(&Color::new_rgba(0, 255, 0, 150));

        Game {
            resources: resources,
            music_manager: music_manager,
            window: window,
            state_stack: state_stack,
            pm: ParticleManager::new(),
            clock: Clock::new(),
            pf_clock: Clock::new(),
            train: Train::new(),
            actors: vec![],
            enemies: vec![],
            selected_actors: vec![],
            menu: Menu { buttons: vec![] },
            world: World {
                bgs: vec![],
                rails: vec![],
            },
            camera: Camera::new(),
            tile_selection: tile_selection,
            paused_text: Text::new().unwrap(),
            is_paused: false,
            selection_rect: selection_rect,
            selecting: false,
            handles: vec![],
            channel: channel(),
        }
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        self.init();

        while self.window.is_open() {
            self.process_events();
            self.update();
            self.render();
        }
        Ok(())
    }

    /// Initializes all the game objects (Example: run this to start a new game)
    fn init(&mut self) {
        self.camera.game = self.window.get_default_view();

        self.world.init(&self.resources.tm);

        self.menu.buttons.push(Button::new(self.resources.fm.get(FontId::Arial),
                                           ButtonType::Resume,
                                           &Vector2f::new(150., 180.)));
        self.menu.buttons.push(Button::new(self.resources.fm.get(FontId::Arial),
                                           ButtonType::Quit,
                                           &Vector2f::new(150., 180. + 80.)));


        //--------

        self.train.init(700., 0.8); // top speed, accel
        self.train.wagons.push(Wagon::new(&self.resources.tm, 8, 3));

        let mut y_size: i32 = 5;
        for x in 0..10 {
            let mut new_wag = Wagon::new(&self.resources.tm, 8, 3);
            y_size += if x % 2 == 0 { 2 } else { -2 };

            self.train.wagons.last_mut().unwrap().connect(&mut new_wag, &self.resources.tm);

            self.train.wagons.push(new_wag);
        }

        self.train.rebuild_pfgrids();

        self.train.set_position2f(2. * TILE_SIZE_X as f32, 2. * TILE_SIZE_Y as f32);

        //---------

        self.actors = vec![Actor::new(&self.resources.tm.get(TextureId::Actor)),
                           Actor::new(&self.resources.tm.get(TextureId::Actor)),
                           Actor::new(&self.resources.tm.get(TextureId::Actor)),
                           Actor::new(&self.resources.tm.get(TextureId::Actor)),
                           Actor::new(&self.resources.tm.get(TextureId::Actor))];

        let mut offset = 0.;
        for a in self.actors.iter_mut() {
            a.sprite.move2f(offset, offset);
            offset += 64.;
        }

        self.enemies = vec![Enemy::new(&self.resources.tm.get(TextureId::Enemy)),
                            Enemy::new(&self.resources.tm.get(TextureId::Enemy))];

        for (x, e) in self.enemies.iter_mut().enumerate() {
            e.sprite.move2f((x as u32 * TILE_SIZE_X) as f32, 0.);
        }

        //        self.train.screech_snd = Some(self.music_manager.get_mut(MusicId::Screech));
        self.paused_text.set_font(&self.resources.fm.get(FontId::Joystix));
        self.paused_text.set_string("PAUSED");
        self.paused_text.set_character_size(36);
        self.paused_text.set_position2f(WINDOW_SIZE_X as f32 / 2. - 100., WINDOW_SIZE_Y as f32 / 2. + 300.);
    }

    fn process_events(&mut self) {
        for event in self.window.events() {
            match *self.state_stack.top().unwrap() {
                StateType::Playing => {
                    // Camera movement
                    if MouseButton::Middle.is_pressed() {
                        self.camera.move_by_mouse(&self.window
                            .map_pixel_to_coords_current_view(&self.window.get_mouse_position()));

                        self.window.set_view(&self.camera.game);
                    }
                    self.camera.mouse_pos_old = self.window
                        .map_pixel_to_coords_current_view(&self.window.get_mouse_position());

                    match event {
                        event::Closed => self.window.close(),
                        event::MouseMoved { x, y, .. } => {
                            let coords = self.window
                                .map_pixel_to_coords_current_view(&Vector2i::new(x, y));

                            self.tile_selection.set_position2f(((TILE_SIZE_X as u32) * (coords.x as u32 / TILE_SIZE_X)) as f32,
                                                               ((TILE_SIZE_Y as u32) * (coords.y as u32 / TILE_SIZE_Y)) as f32);

                            if self.selecting {
                                let rect_pos = self.selection_rect.get_position();
                                self.selection_rect.set_size2f(-(rect_pos.x - x as f32), -(rect_pos.y - y as f32));
                            }
                        }
                        event::MouseButtonPressed { button, .. } => {
                            match button {
                                MouseButton::Left => {
                                    self.selecting = true;
                                    self.selection_rect.set_size2f(0., 0.);
                                    let coords = self.window
                                        .map_pixel_to_coords_current_view(&self.window
                                                                          .get_mouse_position());
                                    self.selection_rect.set_position(&coords);


                                    // if let Some(a) = actor_to_unselect {
                                    //     self.actors[a].sprite.set_color(&Color::white());
                                    // }

                                    // if let Some(sa) = self.selected_actor {
                                    //     let click_pos = self.window
                                    //         .map_pixel_to_coords_current_view(&self.window
                                    //                                           .get_mouse_position());
                                    //     // open/close door
                                    //     let mut pfgrids_must_be_rebuilt = false;
                                    //     let train_origin = self.train.get_origin();
                                    //     'all: for w in self.train.wagons.iter_mut() {
                                    //         for t in w.tiles.iter_mut() {
                                    //             for t in t.iter_mut() {
                                    //                 if let TileType::Door(ref dir) = t.tile_type {
                                    //                     let actor = &mut self.actors[sa];

                                    //                     if t.sprite.get_global_bounds().contains(click_pos) {
                                    //                         if let Some(n) = actor.number_of_steps_to(&self.train.pfgrid_all, &train_origin, click_pos) {
                                    //                             let required_n = if actor.inside_wagon {
                                    //                                 1
                                    //                             } else {
                                    //                                 0
                                    //                             };

                                    //                             if n == required_n {
                                    //                                 if t.is_solid {
                                    //                                     // open it
                                    //                                     t.is_solid = false;
                                    //                                     t.sprite.set_texture(&self.resources.tm.get(TextureId::DoorOpen(dir.clone())), false);
                                    //                                 } else {
                                    //                                     t.is_solid = true;
                                    //                                     t.sprite.set_texture(&self.resources.tm.get(TextureId::DoorClosed(dir.clone())), false);
                                    //                                 }
                                    //                                 pfgrids_must_be_rebuilt = true;
                                    //                             } else {
                                    //                                 let (pfgrid_to_use, dest) = if actor.inside_wagon {
                                    //                                     (self.train.pfgrid_in.clone(), match *dir {
                                    //                                         Direction::North => Vector2f::new(click_pos.x, click_pos.y + (TILE_SIZE_Y * 1) as f32),
                                    //                                         Direction::South => Vector2f::new(click_pos.x, click_pos.y - (TILE_SIZE_Y * 1) as f32),
                                    //                                         Direction::East => Vector2f::new(click_pos.x - (TILE_SIZE_X * 1) as f32, click_pos.y),
                                    //                                         Direction::West => Vector2f::new(click_pos.x + (TILE_SIZE_X * 1) as f32, click_pos.y),})
                                    //                                 } else {
                                    //                                     (self.train.pfgrid_out.clone(), match *dir {
                                    //                                         _ => Vector2f::new(click_pos.x, click_pos.y),
                                    //                                     })
                                    //                                 };


                                    //                                 // TODO:
                                    //                                 let start = actor.sprite.get_position();
                                    //                                 let maybe_path = compute_path(start, pfgrid_to_use, train_origin, dest);
                                    //                                 if let Some(path) = maybe_path {
                                    //                                     actor.set_path(path, train_origin);
                                    //                                 }
                                    //                             }
                                    //                             break 'all;
                                    //                         }
                                    //                     }
                                    //                 }
                                    //             }
                                    //         }
                                    //     }
                                    //     if pfgrids_must_be_rebuilt {
                                    //         self.train.rebuild_pfgrids();
                                    //     }
                                    // }
                                }
                                MouseButton::Right => {
                                    if !self.selected_actors.is_empty() {
                                        println!("{:?}", self.train.total_size);
                                        let click_pos = self.window
                                            .map_pixel_to_coords_current_view(&self.window
                                                                              .get_mouse_position());
                                        let train_origin = self.train.get_origin();

                                        for (idx, sa) in self.selected_actors.iter().enumerate() {
                                            let mut actor = &mut self.actors[*sa];

                                            let pfgrid_to_use = if actor.inside_wagon {
                                                self.train.pfgrid_in.clone()
                                            } else {
                                                self.train.pfgrid_out.clone()
                                            };

                                            let start = actor.sprite.get_position();
                                            let send = self.channel.0.clone();
                                            self.handles.push((idx, thread::spawn(move || {
                                                let path = compute_path(start,
                                                             pfgrid_to_use,
                                                             train_origin,
                                                             click_pos);
                                                send.send(idx);
                                                path
                                            })));


                                            // if let Some(path) = maybe_path {
                                            //     actor.set_path(path, train_origin);
                                            // }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        event::MouseButtonReleased { button, .. } => {
                            match button {
                                MouseButton::Left => {
                                    if self.selecting {
                                        self.selecting = false;
                                        println!("selection over!");

                                        // select actor under cursor
                                        let mut actors_to_unselect: Vec<usize> = vec![];
                                        for (i, a) in self.actors.iter_mut().enumerate() {
                                            if a.sprite
                                                .get_global_bounds()
                                                .intersects(&self.selection_rect.get_global_bounds()) != None {
                                                    // TODO:
                                                    //                                                actor_to_unselect = self.selected_actor;
                                                    if !self.selected_actors.contains(&i) {
                                                        a.sprite.set_color(&Color::green());
                                                        self.selected_actors.push(i);
                                                    }
                                                }
                                        }
                                    }

                                }
                                _ => {}
                            }
                        }
                        event::MouseWheelMoved { delta, .. } => {
                            self.camera.zoom(delta);
                            self.window.set_view(&self.camera.game);
//                            self.world.recalculate_drawables(&self.camera.view, &self.window.map_pixel_to_coords_current_view(&Vector2i::new(0, 0)), &self.resources.tm);
                        }
                        event::KeyReleased { code, .. } => {
                            if let Key::Escape = code {
                                self.state_stack.push(StateType::Menu);
                                println!("{:?}", self.state_stack);
                            }

                            if let Key::Space = code {
                                self.is_paused = !self.is_paused;

                                // pause all musics

                                for (_, music) in self.music_manager.resource_map.iter_mut() {
                                    match music.get_status() {
                                        SoundStatus::Playing => music.pause(),
                                        SoundStatus::Paused => music.play(),
                                        _ => {}
                                    }
                                }
                            }

                            if let Key::G = code {
                                self.train.moving = !self.train.moving;
                                {
                                    let mut train_sound = self.music_manager.get_mut(MusicId::Train);
                                    train_sound.set_loop(true);
                                    if train_sound.get_status() == SoundStatus::Stopped {
                                        train_sound.play();
                                    }
                                }

                                {
                                    if !self.train.moving {
                                        let mut screech_sound = self.music_manager.get_mut(MusicId::Screech);
                                        screech_sound.set_loop(true);
                                        screech_sound.play();
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                StateType::Menu => {
                    match event {
                        event::KeyReleased { code, .. } => {
                            match code {
                                Key::Escape => {
                                    self.state_stack.pop();
                                    println!("{:?}", self.state_stack);
                                }
                                _ => {}
                            }
                        }
                        event::MouseMoved { x, y, .. } => {
                            for button in &mut self.menu.buttons {
                                let x = x as f32;
                                let y = y as f32;
                                if x > button.text.get_position().x &&
                                   x <
                                   (button.text.get_position().x +
                                    button.text.get_local_bounds().width) &&
                                   y > button.text.get_position().y &&
                                   y <
                                   (button.text.get_position().y +
                                    button.text.get_local_bounds().height * 2.) {
                                    // <- *2. because Text bounding box is broken - SFML bug?
                                    button.text.set_color(&Color::green());
                                } else {
                                    button.text.set_color(&Color::white());
                                }
                            }
                        }
                        event::MouseButtonReleased { button, .. } => {
                            match button {
                                MouseButton::Left => {
                                    for button in &self.menu.buttons {
                                        // check if the button is literally green
                                        // TODO: change this to something better
                                        if button.text.get_color().0 ==
                                           Color::new_rgb(0, 255, 0).0 {
                                            match button.button_type {
                                                ButtonType::Quit => {
                                                    self.window.close();
                                                }
                                                ButtonType::Resume => {
                                                    self.state_stack.pop();
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                StateType::GameOver => {
                    //     match event {
                    //         event::Closed => {
                    //             window.close();
                    //         }
                    //         event::KeyReleased { code, .. } => {
                    //             match code {
                    //                 Key::R => {
                    //                     // reset the game
                    //                     state_stack.pop();

                    //                 }
                    //                 _ => {}
                    //             }
                    //         }
                    //         _ => {}
                    //     }
                }
            }
        }
    }

    fn update(&mut self) {
        let time = self.clock.restart();
        match *self.state_stack.top().unwrap() {
            StateType::Playing => {
                let train_origin = self.train.get_origin();
                for recv in self.channel.1.try_iter() {
                    println!("recv val {}, sa len: {}", recv, self.selected_actors.len());
                    let true_index = self.handles.iter().position(|ref x| x.0 == recv).unwrap();
                    self.actors[self.selected_actors[recv]].set_path(&mut self.handles.remove(true_index).1.join().unwrap().unwrap(),
                                                                     train_origin);
                }

                if !self.is_paused {
                    let dt = time.as_seconds();

                    for a in self.actors.iter_mut() {
                        a.update_movement(&self.train.wagons, dt);
                    }

                    let time_elapsed = self.pf_clock.get_elapsed_time();
                    if time_elapsed.as_seconds() >= 1. {
                        let train_origin = self.train.get_origin();

                        let enemy_destination = self.train.wagons[0].tiles[0][2].sprite.get_position();


                        // let (tx, rx) = mpsc::channel();

                        for (idx, e) in self.enemies.iter_mut().enumerate() {

                            // TODO: WRONG!
                        //     let enemy_pos = e.sprite.get_position().clone();

                        //     let pfgrid = self.train.pfgrid_out.clone();

                        //     let tx = tx.clone();

                        //     self.handles.push((idx, thread::spawn(move || {
                        //         let path_result = compute_path(enemy_pos,
                        //                                        pfgrid,
                        //                                        train_origin,
                        //                                        enemy_destination);
                        //         tx.send(idx).unwrap();
                        //         path_result
                        //     })));
                            e.update_movement(&self.train.wagons, dt);
                        }

                        // let train_origin = self.train.get_origin();
                        // TODO: WRONG!
                        // let all_threads_finished = true;
                        // for _ in 0..self.enemies.len() {
                        //     let idx = rx.recv().unwrap();
                        //     println!("{} started", idx);
                        //     self.enemies[idx].set_path(self.handles.swap_remove(0).1.join().unwrap().unwrap(),
                        //                                train_origin);
                        //     println!("{} finished", idx);
                        // }

                        // for &mut (idx, ref handle) in self.handles.iter_mut() {
                        //     match handle.join() {
                        //         Ok(Some(path)) => {
                        //             self.enemies[idx].set_path(path, train_origin);
                        //         }
                        //         Ok(None) => {
                        //             println!("No path available!");
                        //         }
                        //         Err(_) => {} // dont care
                        //     }
                        // }
                    }




                    // for e in self.enemies.iter_mut() {
                    //     if self.train.current_speed > 0. {
                    //         e.set_path(&self.train.pfgrid_out, &train_origin, self.train.wagons[0].tiles[0][2].sprite.get_position());
                    //     }
                    //
//                    }

                    self.world.update(dt * -self.train.current_speed);

                    self.train.update();

                    // sounds
                    if self.train.current_speed > 0. {
                        {
                            let mut train_sound = self.music_manager.get_mut(MusicId::Train);
                            train_sound.set_volume(100. * self.train.current_speed / self.train.top_speed);

                            if !self.train.moving && self.train.current_speed <= self.train.top_speed / 4. {
                                train_sound.stop();
                            }
                        }
                        {
                            if !self.train.moving {
                                let mut screech_sound = self.music_manager.get_mut(MusicId::Screech);
                                screech_sound.set_volume(70. * self.train.current_speed / self.train.top_speed);
                            }
                        }
                    } else {
                        // {
                        //     let mut train_sound = self.music_manager.get_mut(MusicId::Train);
                        //     train_sound.stop();
                        // }
                        {
                            let mut screech_sound = self.music_manager.get_mut(MusicId::Screech);
                            screech_sound.stop();
                        }
                    }

                    //

                    for a in self.actors.iter_mut() {
                        if !a.inside_wagon {
                            //TODO: add collision checking to this (refactor what is above into a checking function)
                            a.sprite.move2f(dt * -self.train.current_speed, 0.);
                        }
                    }

                    for e in self.enemies.iter_mut() {
                        if !e.inside_wagon {
                            //TODO add collision checking to this (refactor what is above into a checking function)
                            e.sprite.move2f(dt * -self.train.current_speed, 0.);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn render(&mut self) {
        match *self.state_stack.top().unwrap() {
            StateType::Playing => {
                self.window.set_view(&self.camera.game);
                // Clear the window
                self.window.clear(&Color::yellow());

                for bg in self.world.bgs.iter() {
                    self.window.draw(bg);
                }

                for rail in self.world.rails.iter() {
                    self.window.draw(rail);
                }

                for w in self.train.wagons.iter() {
                    // culling boys
                    let wagon_bound = FloatRect::new(w.get_origin().x,
                                                     w.get_origin().y,
                                                     (w.tiles[0].len() as u32 * TILE_SIZE_X) as f32,
                                                     (w.tiles.len() as u32 * TILE_SIZE_Y) as f32);

                    let view = &self.camera.game;
                    let view_rect = FloatRect::new(view.get_center().x - view.get_size().x / 2.,
                                                   view.get_center().y - view.get_size().y / 2.,
                                                   view.get_size().x,
                                                   view.get_size().y);
                    if let Some(_) = view_rect.intersects(&wagon_bound) {
                        self.window.draw(w);
                    }
                }

                for a in self.actors.iter() {
                    // draw path
                    let steps = Vec::from(a.move_seq.clone());
                    for step in steps.windows(2) {
                        let mut va = VertexArray::new().unwrap();
                        va.set_primitive_type(PrimitiveType::sfLines);
                        va.append(&Vertex::new_with_pos_color(&step[0], &Color::green()));
                        va.append(&Vertex::new_with_pos_color(&step[1], &Color::green()));
                        self.window.draw(&va);
                    }

                    self.window.draw(&a.sprite);
                }

                for e in self.enemies.iter() {
                    self.window.draw(&e.sprite);
                }

                if self.selecting {
                    self.window.draw(&self.selection_rect);
                }

                // if let Some(selected_actor) = self.selected_actor {
                //     for (i, t) in if self.actors[selected_actor].inside_wagon {
                //         self.train.pfgrid_in.grid.iter().enumerate()
                //     } else {
                //         self.train.pfgrid_out.grid.iter().enumerate()
                //     } {
                //         for (j, t) in t.iter().enumerate() {
                //             let train_origin = self.train.get_origin();

                //             let mut shape = RectangleShape::new().unwrap();
                //             shape.set_size2f(64., 64.);
                //             shape.set_position2f(i as f32 * 64. + train_origin.x - 2. * 64.,
                //                                  j as f32 * 64. + train_origin.y - 2. * 64.);

                //             if t.walkable {
                //                 shape.set_fill_color(&Color::new_rgba(0, 255, 0, 120));
                //             } else {
                //                 shape.set_fill_color(&Color::new_rgba(255, 0, 0, 120));
                //             }

                //             self.window.draw(&shape);
                //         }
                //     }
                // }
                if !self.selected_actors.is_empty() {
                    self.window.draw(&self.tile_selection);
                }

                if self.is_paused {
                    // ui view
                    self.window.set_view(&self.camera.ui);
                    self.window.draw(&self.paused_text);
                    self.window.set_view(&self.camera.game);
                }

            }
            StateType::Menu => {
                self.window.clear(&Color::black());

                for button in &self.menu.buttons {
                    self.window.draw(&button.text);
                }
            }
            _ => {}
        }

        self.window.display();
    }
}
