use rand::Rng;

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

pub struct Game<'a> {
    resources: &'a Resources,
    music_manager: &'a mut MusicManager,
    window: RenderWindow,
    state_stack: StateStack,
    pm: ParticleManager<'a>,
    clock: Clock,
    train: Train<'a>,
    actors: Vec<Actor<'a>>,
    enemies: Vec<Enemy<'a>>,
    selected_actor: Option<usize>,
    menu: Menu<'a>,
    world: World<'a>,
    camera: Camera,
    tile_selection: RectangleShape<'a>,
}

impl<'a> Game<'a> {
    pub fn new(resources: &'a Resources, music_manager: &'a mut MusicManager) -> Self {
        // Create the window of the application
        let mut window = RenderWindow::new(VideoMode::new_init(WINDOW_SIZE_X, WINDOW_SIZE_Y, 32),
                                           "Train",
                                           window_style::CLOSE,
                                           &ContextSettings::default())
            .unwrap();

        window.set_framerate_limit(60);
        window.set_vertical_sync_enabled(true);

        let mut state_stack = StateStack::new();
        state_stack.push(StateType::Playing);

        let mut tile_selection = RectangleShape::new().unwrap();
        tile_selection.set_size2f(TILE_SIZE_X as f32, TILE_SIZE_Y as f32);
        tile_selection.set_fill_color(&Color::new_rgba(255, 255, 0, 140));

        Game {
            resources: resources,
            music_manager: music_manager,
            window: window,
            state_stack: state_stack,
            pm: ParticleManager::new(),
            clock: Clock::new(),
            train: Train::new(),
            actors: vec![],
            enemies: vec![],
            selected_actor: None,
            menu: Menu { buttons: vec![] },
            world: World {
                bgs: vec![],
                rails: vec![],
            },
            camera: Camera::new(),
            tile_selection: tile_selection,
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
        self.camera.view = self.window.get_default_view();

        self.world.init(&self.resources.tm);

        self.menu.buttons.push(Button::new(self.resources.fm.get(FontId::Arial),
                                           ButtonType::Resume,
                                           &Vector2f::new(150., 180.)));
        self.menu.buttons.push(Button::new(self.resources.fm.get(FontId::Arial),
                                           ButtonType::Quit,
                                           &Vector2f::new(150., 180. + 80.)));


        self.train.init(700., 0.8); // top speed, accel

        // <test>
        self.train.wagons.push(Wagon::new(&self.resources.tm, 2, 7));
        self.train.wagons.push(Wagon::new(&self.resources.tm, 4, 5));
        self.train.wagons.push(Wagon::new(&self.resources.tm, 6, 7));


//        self.train.wagons[1].tiles[6][2].sprite.set_texture(&self.resources.tm.get(TextureId::DoorSouth), false);
//        self.train.wagons[1].tiles[6][2].is_solid = false;
        self.train.wagons[0].set_position2f(TILE_SIZE_X as f32 * 14., TILE_SIZE_Y as f32 * 2.);
        // </test>

        {
            let (a, b) = self.train.wagons.split_at_mut(1);
            a[0].connect(&mut b[0], &self.resources.tm);
        }
        {
            let (a, b) = self.train.wagons.split_at_mut(2);
            a[1].connect(&mut b[0], &self.resources.tm);
        }

        self.train.rebuild_pfgrids();

        self.actors = vec![Actor::new(&self.resources.tm.get(TextureId::Actor))];//, Actor::new(), Actor::new(), Actor::new()];
        self.enemies = vec![Enemy::new(&self.resources.tm.get(TextureId::Enemy)),Enemy::new(&self.resources.tm.get(TextureId::Enemy)),Enemy::new(&self.resources.tm.get(TextureId::Enemy)),Enemy::new(&self.resources.tm.get(TextureId::Enemy))];

        //        self.train.screech_snd = Some(self.music_manager.get_mut(MusicId::Screech));
    }

    fn process_events(&mut self) {
        for event in self.window.events() {
            match *self.state_stack.top().unwrap() {
                StateType::Playing => {
                    // Camera movement
                    if MouseButton::Middle.is_pressed() {
                        self.camera.move_by_mouse(&self.window
                            .map_pixel_to_coords_current_view(&self.window.get_mouse_position()));

                        self.window.set_view(&self.camera.view);
                    }
                    self.camera.mouse_pos_old = self.window
                        .map_pixel_to_coords_current_view(&self.window.get_mouse_position());

                    match event {
                        event::Closed => self.window.close(),
                        event::MouseMoved { x, y, .. } => {
                            let coords = self.window
                                .map_pixel_to_coords_current_view(&Vector2i::new(x, y));
                            if let Some(_) = self.selected_actor {
                                self.tile_selection.set_position2f(((TILE_SIZE_X as u32) * (coords.x as u32 / TILE_SIZE_X)) as f32,
                                                                   ((TILE_SIZE_Y as u32) * (coords.y as u32 / TILE_SIZE_Y)) as f32);
                            }
                        }
                        event::MouseButtonPressed { button, .. } => {
                            match button {
                                MouseButton::Left => {
                                    // select actor under cursor
                                    let mut actor_to_unselect: Option<usize> = None;
                                    for (i, a) in self.actors.iter_mut().enumerate() {
                                        let coords = self.window
                                            .map_pixel_to_coords_current_view(&self.window
                                                .get_mouse_position());
                                        if a.sprite
                                            .get_global_bounds()
                                            .contains(coords.to_vector2f()) {
                                                actor_to_unselect = self.selected_actor;
                                                a.sprite.set_color(&Color::green());
                                                self.selected_actor = Some(i);
                                                break;
                                        }
                                    }
                                    if let Some(a) = actor_to_unselect {
                                        self.actors[a].sprite.set_color(&Color::white());
                                    }

                                    // open/close door
                                    let mut pfgrids_must_be_rebuilt = false;
                                    for w in self.train.wagons.iter_mut() {
                                        for t in w.tiles.iter_mut() {
                                            for t in t.iter_mut() {
                                                if let TileType::Door(ref dir) = t.tile_type {
                                                    if t.is_solid {
                                                        // open it
                                                        t.is_solid = false;
                                                        t.sprite.set_texture(&self.resources.tm.get(TextureId::DoorOpen(dir.clone())), false);
                                                    } else {
                                                        t.is_solid = true;
                                                        t.sprite.set_texture(&self.resources.tm.get(TextureId::DoorClosed(dir.clone())), false);
                                                    }
                                                    pfgrids_must_be_rebuilt = true;
                                                }
                                            }
                                        }
                                    }
                                    if pfgrids_must_be_rebuilt {
                                        self.train.rebuild_pfgrids();
                                    }
                                }
                                MouseButton::Right => {
                                    if let Some(selected_actor) = self.selected_actor {
                                        println!("{:?}", self.train.total_size);
                                        let click_pos = self.window
                                            .map_pixel_to_coords_current_view(&self.window
                                                                              .get_mouse_position());

                                        let mut actor = &mut self.actors[selected_actor];

                                        let pfgrid_to_use = if actor.inside_wagon {
                                            &self.train.pfgrid_in
                                        } else {
                                            &self.train.pfgrid_out
                                        };

                                        let train_origin = self.train.get_origin();
                                        actor.set_path(pfgrid_to_use,
                                                       &train_origin,
                                                       click_pos);
                                    }
                                }
                                _ => {}
                            }
                        }
                        event::MouseWheelMoved { delta, .. } => {
                            self.camera.zoom(delta);
                            self.window.set_view(&self.camera.view);
//                            self.world.recalculate_drawables(&self.camera.view, &self.window.map_pixel_to_coords_current_view(&Vector2i::new(0, 0)), &self.resources.tm);
                        }
                        event::KeyReleased { code, .. } => {
                            if let Key::Escape = code {
                                self.state_stack.push(StateType::Menu);
                                println!("{:?}", self.state_stack);
                            }
                            if let Key::Space = code {
                                self.train.moving = !self.train.moving;
                                // {
                                //     let mut train_sound = self.music_manager.get_mut(MusicId::Train);
                                //     train_sound.set_loop(true);
                                //     train_sound.set_volume(25.);
                                //     if train_sound.get_status() == SoundStatus::Stopped {
                                //         train_sound.play();
                                //     } else {
                                //         train_sound.stop();
                                //     }
                                // }

                                // {
                                //     if !self.train.moving {
                                //         let mut screech_sound = self.music_manager.get_mut(MusicId::Screech);
                                //         screech_sound.set_loop(true);
                                //         screech_sound.set_volume(25.);
                                //         if screech_sound.get_status() == SoundStatus::Stopped {
                                //             screech_sound.play();
                                //         } else {
                                //             screech_sound.stop();
                                //         }
                                //     }
                                // }
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
                let dt = time.as_seconds();

                for a in self.actors.iter_mut() {
                    a.update_movement(&self.train.wagons, dt);
                }

                let train_origin = self.train.get_origin();
                for e in self.enemies.iter_mut() {
                    if self.train.current_speed > 0. {
                        e.set_path(&self.train.pfgrid_out, &train_origin, self.train.wagons[0].tiles[0][2].sprite.get_position());
                    }
                    e.update_movement(&self.train.wagons, dt);
                }

                self.world.update(dt * -self.train.current_speed);

                self.train.update();

                for a in self.actors.iter_mut() {
                    if !a.inside_wagon {
                        // add collision checking to this (refactor what is above into a checking function)
                        a.sprite.move2f(dt * -self.train.current_speed, 0.);
                    }
                }

                for e in self.enemies.iter_mut() {
                    if !e.inside_wagon {
                        // add collision checking to this (refactor what is above into a checking function)
                        e.sprite.move2f(dt * -self.train.current_speed, 0.);
                    }
                }
            }
            _ => {}
        }
    }

    fn render(&mut self) {
        match *self.state_stack.top().unwrap() {
            StateType::Playing => {
                // Clear the window
                self.window.clear(&Color::yellow());

                for bg in self.world.bgs.iter() {
                    self.window.draw(bg);
                }

                for rail in self.world.rails.iter() {
                    self.window.draw(rail);
                }



                for w in self.train.wagons.iter() {
                    self.window.draw(w);
                }

                for a in self.actors.iter() {
                    self.window.draw(&a.sprite);
                }

                for e in self.enemies.iter() {
                    self.window.draw(&e.sprite);
                }

                if let Some(selected_actor) = self.selected_actor {
                    for (i, t) in if self.actors[selected_actor].inside_wagon {
                        self.train.pfgrid_in.grid.iter().enumerate()
                    } else {
                        self.train.pfgrid_out.grid.iter().enumerate()
                    } {
                        for (j, t) in t.iter().enumerate() {
                            let train_origin = self.train.get_origin();

                            let mut shape = RectangleShape::new().unwrap();
                            shape.set_size2f(64., 64.);
                            shape.set_position2f(i as f32 * 64. + train_origin.x - 2. * 64.,
                                                 j as f32 * 64. + train_origin.y - 2. * 64.);

                            if t.walkable {
                                shape.set_fill_color(&Color::new_rgba(0, 255, 0, 120));
                            } else {
                                shape.set_fill_color(&Color::new_rgba(255, 0, 0, 120));
                            }

                            self.window.draw(&shape);
                        }
                    }
                }

                self.window.draw(&self.tile_selection);
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
