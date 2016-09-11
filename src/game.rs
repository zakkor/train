// // Create the window of the application
// let mut window = RenderWindow::new(VideoMode::new_init(WINDOW_SIZE_X, WINDOW_SIZE_Y, 32),
//                                    "Train",
//                                    window_style::CLOSE,
//                                    &ContextSettings::default())
//     .unwrap();
// window.set_framerate_limit(60);
// window.set_vertical_sync_enabled(true);

// let mut font_manager = FontManager::new();
// font_manager.load(FontId::Arial, "res/arial.ttf");
// font_manager.load(FontId::Joystix, "res/joystix-mono.ttf");

// let mut view = window.get_default_view();
// //view.set_center2f(0.5, 0.5);
// window.set_view(&view);



// // let mut game_over_text = Text::new().unwrap();
// // game_over_text.set_font(font_manager.get(FontId::Arial));
// // game_over_text.set_position(&Vector2f::new(1280. / 2. - 175., 250.));
// // game_over_text.set_color(&Color::white());
// // game_over_text.set_character_size(60);
// // game_over_text.set_string("GAME OVER!");


// let mut tex_man = TextureManager::new();
// tex_man.load(TextureId::Floor, "res/floor.png");
// tex_man.load(TextureId::WallTop, "res/wall_top.png");
// tex_man.load(TextureId::WallBottom, "res/wall_bottom.png");
// tex_man.load(TextureId::WallLeft, "res/wall_left.png");
// tex_man.load(TextureId::WallRight, "res/wall_right.png");
// tex_man.load(TextureId::CornerTopLeft, "res/corner_topleft.png");
// tex_man.load(TextureId::CornerTopRight, "res/corner_topright.png");
// tex_man.load(TextureId::CornerBottomLeft, "res/corner_bottomleft.png");
// tex_man.load(TextureId::CornerBottomRight, "res/corner_bottomright.png");
// tex_man.load(TextureId::ConnectorTop, "res/connector_top.png");
// tex_man.load(TextureId::ConnectorBottom, "res/connector_bottom.png");
// tex_man.load(TextureId::WallConnectedTop, "res/wall_connected_top.png");
// tex_man.load(TextureId::WallConnectedBottom, "res/wall_connected_bottom.png");
// tex_man.load(TextureId::Background, "res/bg.png");


// let mut state_stack = StateStack::new();
// state_stack.push(StateType::Playing);

// let mut particle_manager = ParticleManager::new();


// // // view
// // let mut view = View::new_init(&Vector2f::new(1280. / 2., 720. / 2.),
// //                               &Vector2f::new(1280., 720.))
// //     .unwrap();
// // window.set_view(&view);


// // delta time
// let mut clock = Clock::new();

// let mut wagons = vec![Wagon::new(&tex_man, 5, 7),
//                       Wagon::new(&tex_man, 5, 5),
//                       Wagon::new(&tex_man, 3, 3),
// ];


// wagons[0].set_position2f(788., 225.);
// {
//     let (a, b) = wagons.split_at_mut(1);
//     a[0].connect(&mut b[0], &tex_man);
// }
// {
//     let (a, b) = wagons.split_at_mut(2);
//     a[1].connect(&mut b[0], &tex_man);
// }

// wagons[1].tiles[0][3] = Tile::new();


// let mut bg1 = Sprite::new_with_texture(&tex_man.get(TextureId::Background)).unwrap();
// let mut bg2 = bg1.clone();
// bg2.set_position2f(1600., 0.);


// // menu
// let mut menu = Menu {
//     buttons: vec![Button::new(font_manager.get(FontId::Joystix),
//                               ButtonType::Resume,
//                               &Vector2f::new(150., 180.)),
//                   Button::new(font_manager.get(FontId::Joystix),
//                               ButtonType::Quit,
//                               &Vector2f::new(150., 180. + 80.))],
// };

// let mut mouse_pos_old = Vector2f::new(WINDOW_SIZE_X as f32 / 2., WINDOW_SIZE_Y as f32 / 2.);

// // TODO: move to Train
// let mut current_speed = 0.;
// let top_speed = 1000.;
// let accel = 5.;

// let mut moving = false;



// let mut actors = vec![Player::new(), Player::new()];

// let mut selected_actor = 0;

// while window.is_open() {
//     // ___________________ EVENTS_BEGIN ______________//
//     {
//         for event in window.events() {
//             match *state_stack.top().unwrap() {
//                 StateType::Playing => {
//                     if MouseButton::Middle.is_pressed() {
//                         let mouse_pos = window.map_pixel_to_coords_current_view(&window.get_mouse_position());
//                         let move_factor = Vector2f::new(mouse_pos_old.x - mouse_pos.x,
//                                                         mouse_pos_old.y - mouse_pos.y);

//                         view.move_(&move_factor);
//                         window.set_view(&view);
//                     }
//                     mouse_pos_old = window.map_pixel_to_coords_current_view(&window.get_mouse_position());



//                     match event {
//                         event::Closed => window.close(),
//                         event::MouseMoved { x, .. } => {}
//                         event::MouseButtonPressed { button, .. } => {
//                             match button {
//                                 MouseButton::Left => {
//                                     // select actor under cursor
//                                     let mut actor_to_unselect: Option<usize> = None;
//                                     for (i, a) in actors.iter_mut().enumerate() {
//                                         if a.shape.get_global_bounds().contains(window.get_mouse_position().to_vector2f()) {
//                                             actor_to_unselect = Some(selected_actor);
//                                             a.shape.set_fill_color(&Color::green());
//                                             selected_actor = i;
//                                             break;
//                                         }
//                                     }
//                                     if let Some(a) = actor_to_unselect {
//                                         actors[a].shape.set_fill_color(&Color::red());
//                                     }
//                                 }
//                                 MouseButton::Right => {
//                                     let move_to = window.get_mouse_position().to_vector2f();
//                                     let current_pos = actors[selected_actor].shape.get_position();
//                                     let mut move_dir = Vector2f::new(move_to.x - current_pos.x, move_to.y - current_pos.y);
//                                     let vec_len = (move_dir.x.powi(2) + move_dir.y.powi(2)).sqrt().abs();
//                                     move_dir.x = move_dir.x / vec_len;
//                                     move_dir.y = move_dir.y / vec_len;
//                                     actors[selected_actor].move_dir = move_dir;
//                                 }

//                                 _ => {}
//                             }
//                         }
//                         event::MouseWheelMoved { delta, .. } => {
//                             let zoom_factor = match delta < 0 {
//                                 true => 1.1,
//                                 false => 0.9
//                             };

//                             view.zoom(zoom_factor);
//                             window.set_view(&view);
//                         }
//                         event::KeyReleased { code, .. } => {
//                             if let Key::Escape = code {
//                                 state_stack.push(StateType::Menu);
//                                 println!("{:?}", state_stack);
//                             }
//                             if let Key::Space = code {
//                                 moving = !moving;
//                             }
//                             if let Key::P = code {

//                             }
//                         }
//                         _ => {
//                             // do nothing
//                         }
//                     }
//                 }
//                 StateType::Menu => {
//                     match event {
//                         event::KeyReleased { code, .. } => {
//                             match code {
//                                 Key::Escape => {
//                                     state_stack.pop();
//                                     println!("{:?}", state_stack);
//                                 }
//                                 _ => {}
//                             }
//                         }
//                         event::MouseMoved { x, y, .. } => {
//                             for button in &mut menu.buttons {
//                                 let x = x as f32;
//                                 let y = y as f32;
//                                 if x > button.text.get_position().x &&
//                                     x <
//                                     (button.text.get_position().x +
//                                      button.text.get_local_bounds().width) &&
//                                     y > button.text.get_position().y &&
//                                     y <
//                                     (button.text.get_position().y +
//                                      button.text.get_local_bounds().height * 2.) {
//                                         // <- *2. because Text bounding box is broken - SFML bug?
//                                         button.text.set_color(&Color::green());
//                                     } else {
//                                         button.text.set_color(&Color::white());
//                                     }
//                             }
//                         }
//                         event::MouseButtonReleased { button, .. } => {
//                             match button {
//                                 MouseButton::Left => {
//                                     for button in &menu.buttons {
//                                         // TODO: add hover
//                                         if true {
//                                             match button.button_type {
//                                                 ButtonType::Quit => {
//                                                     window.close();
//                                                 }
//                                                 ButtonType::Resume => {
//                                                     state_stack.pop();
//                                                 }
//                                             }
//                                         }
//                                     }
//                                 }
//                                 _ => {}
//                             }
//                         }
//                         _ => {}
//                     }
//                 }
//                 StateType::GameOver => {
//                     match event {
//                         event::Closed => {
//                             window.close();
//                         }
//                         event::KeyReleased { code, .. } => {
//                             match code {
//                                 Key::R => {
//                                     // reset the game
//                                     state_stack.pop();

//                                 }
//                                 _ => {}
//                             }
//                         }
//                         _ => {}
//                     }
//                 }
//             }
//         }

//     }
//     // ___________________ EVENTS_END ______________//

//     let time = clock.restart();
//     match *state_stack.top().unwrap() {
//         StateType::Playing => {
//             {
//                 // // ___________________ UPDATE_BEGIN ______________//
//                 let dt = time.as_seconds();
//                 for (i, a) in actors.iter_mut().enumerate() {
//                     //                                                   a.shape.move2f(dx, dy);

//                     let (dx, dy) = {
//                         let mult = 50. * dt;
//                         (mult * a.move_dir.x, mult * a.move_dir.y)
//                     };

//                     if (dx, dy) != (0., 0.) {
//                         let actor_bounds = a.shape.get_global_bounds();

//                         let desired_pos = FloatRect::new(actor_bounds.left + dx,
//                                                          actor_bounds.top + dy,
//                                                          actor_bounds.width,
//                                                          actor_bounds.height);
//                         let mut ok_to_move = true;
//                         for w in wagons.iter() {
//                             for t in w.tiles.iter() {
//                                 for t in t.iter() {
//                                     if !t.is_solid {
//                                         continue;
//                                     }
//                                     for b in t.bounds.iter() {
//                                         let b = if *b != None {
//                                             b.unwrap()
//                                         }
//                                         else {
//                                             continue;
//                                         };
//                                         if let Some(_) = desired_pos.intersects(
//                                             &FloatRect::new(b.left as f32 + t.sprite.get_position().x,
//                                                             b.top as f32 + t.sprite.get_position().y,
//                                                             b.width as f32,
//                                                             b.height as f32)) {
//                                             ok_to_move = false;
//                                             break;
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                         if ok_to_move && moving {
//                             a.shape.move2f(dx, dy);
//                         }

//                         a.inside_wagon = false;
//                         for w in wagons.iter() {
//                             for t in w.tiles.iter() {
//                                 for t in t.iter() {
//                                     if !t.is_solid && t.sprite.get_global_bounds().contains(a.shape.get_position()) { 
//                                         a.inside_wagon = true;
//                                         break;
//                                     }
//                                 }
//                             }
//                         }
//                     }

//                 }


//                 if bg1.get_position().x <= -(WINDOW_SIZE_X as f32) {
//                     bg1.move2f((WINDOW_SIZE_X * 2) as f32, 0.);
//                 }
//                 if bg2.get_position().x <= -(WINDOW_SIZE_X as f32) {
//                     bg2.move2f((WINDOW_SIZE_X * 2) as f32, 0.);
//                 }

//                 if moving {
//                     if current_speed + accel < top_speed {
//                         current_speed += accel;
//                     }

//                 } else if current_speed > 0. {
//                     current_speed -= accel * 2.;
//                 }

//                 bg1.move2f(dt * -current_speed, 0.);
//                 bg2.move2f(dt * -current_speed, 0.);

//                 for a in actors.iter_mut() {
//                     if !a.inside_wagon {
//                         a.shape.move2f(dt * -current_speed, 0.);
//                     }
//                 }
//                 // ___________________ UPDATE_END ________________//
//             }

//             {
//                 // ___________________ RENDER_BEGIN  _____________//
//                 // Clear the window
//                 window.clear(&Color::black());

//                 window.draw(&bg1);
//                 window.draw(&bg2);

//                 for a in actors.iter() {
//                     if !a.inside_wagon {
//                         window.draw(&a.shape);
//                     }
//                 }

//                 for w in wagons.iter() {
//                     window.draw(w);
//                 }

//                 for a in actors.iter() {
//                     if a.inside_wagon {
//                         window.draw(&a.shape);
//                     }
//                 }


//                 // ____________________ RENDER_END _____________//
//             }
//         }
//         StateType::Menu => {
//             // update
//             {
//                 // don't update anything for now
//             }
//             // render
//             {
//                 window.clear(&Color::black());

//                 for button in &menu.buttons {
//                     window.draw(&button.text);
//                 }
//             }
//         }
//         StateType::GameOver => {
//             // update
//             {
//                 // don't update anything for now
//             }
//             // render
//             {
//                 window.clear(&Color::black());

//             }
//         }
//     }
//     window.display();
// }


use rand::Rng;

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;

use state_stack::*;
use resource_manager::*;
use particle_manager::*;
use actor::Actor;
use menu::*;
use wagon::*;
use game_consts::*;

pub struct Game<'a> {
    resources: &'a Resources,
    window: RenderWindow,
    view: View,
    state_stack: StateStack,
    pm: ParticleManager<'a>,
    clock: Clock,
    wagons: Vec<Wagon<'a>>,
    actors: Vec<Actor<'a>>,
    selected_actor: Option<usize>,
    menu: Menu<'a>,
}

impl<'a> Game<'a> {
    pub fn new(resources: &'a Resources) -> Self {
        // Create the window of the application
        let mut window = RenderWindow::new(VideoMode::new_init(WINDOW_SIZE_X, WINDOW_SIZE_Y, 32),
                                           "Train",
                                           window_style::CLOSE,
                                           &ContextSettings::default()).unwrap();

        window.set_framerate_limit(60);
        window.set_vertical_sync_enabled(true);

        let view = window.get_default_view();
        window.set_view(&view);


        let mut state_stack = StateStack::new();
        state_stack.push(StateType::Playing);

        Game {
            resources: resources,
            window: window,
            view: view,
            state_stack: state_stack,
            pm: ParticleManager::new(),
            clock: Clock::new(),
            wagons: vec![],
            actors: vec![],
            selected_actor: None,
            menu: Menu { buttons: vec![] },
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
        self.menu.buttons.push(Button::new(self.resources.fm.get(FontId::Joystix),
                                           ButtonType::Resume,
                                           &Vector2f::new(150., 180.)));
        self.menu.buttons.push(Button::new(self.resources.fm.get(FontId::Joystix),
                                           ButtonType::Quit,
                                           &Vector2f::new(150., 180. + 80.)));

        self.wagons.push(Wagon::new(&self.resources.tm, 5, 7));
        self.wagons.push(Wagon::new(&self.resources.tm, 3, 7));

        self.wagons[0].set_position2f(300., 300.);

        {
            let (a, b) = self.wagons.split_at_mut(1);
            a[0].connect(&mut b[0], &self.resources.tm);
        }



        self.actors.push(Actor::new());
    }

    fn process_events(&mut self) {
        for event in self.window.events() {
            match *self.state_stack.top().unwrap() {
                StateType::Playing => {
                    // Camera movement
                    // if MouseButton::Middle.is_pressed() {
                    //     let mouse_pos = self.window.map_pixel_to_coords_current_view(&self.window.get_mouse_position());
                    //     let move_factor = Vector2f::new(mouse_pos_old.x - mouse_pos.x,
                    //                                     mouse_pos_old.y - mouse_pos.y);

                    //     view.move_(&move_factor);
                    //     window.set_view(&view);
                    // }
                    // mouse_pos_old = window.map_pixel_to_coords_current_view(&window.get_mouse_position());

                    match event {
                        event::Closed => self.window.close(),
                        // event::MouseMoved { x, .. } => {}
                        event::MouseButtonPressed { button, .. } => {
                            match button {
                                MouseButton::Left => {
                                    // select actor under cursor
                                    let mut actor_to_unselect: Option<usize> = None;
                                    for (i, a) in self.actors.iter_mut().enumerate() {
                                        if a.shape.get_global_bounds().contains(self.window.get_mouse_position().to_vector2f()) {
                                            actor_to_unselect = self.selected_actor;
                                            a.shape.set_fill_color(&Color::green());
                                            self.selected_actor = Some(i);
                                            break;
                                        }
                                    }
                                    if let Some(a) = actor_to_unselect {
                                        self.actors[a].shape.set_fill_color(&Color::red());
                                    }
                                }
                                MouseButton::Right => {
                                    if let Some(selected_actor) = self.selected_actor {
                                        let move_to = self.window.get_mouse_position().to_vector2f();
                                        let current_pos = self.actors[selected_actor].shape.get_position();

                                        let mut move_dir = Vector2f::new(move_to.x - current_pos.x, move_to.y - current_pos.y);
                                        let vec_len = (move_dir.x.powi(2) + move_dir.y.powi(2)).sqrt().abs();

                                        move_dir.x = move_dir.x / vec_len;
                                        move_dir.y = move_dir.y / vec_len;

                                        self.actors[selected_actor].move_dir = move_dir;
                                    }
                                }
                                _ => {}
                            }
                        }
                        // event::MouseWheelMoved { delta, .. } => {
                        //     let zoom_factor = match delta < 0 {
                        //         true => 1.1,
                        //         false => 0.9
                        //     };

                        //     view.zoom(zoom_factor);
                        //     window.set_view(&view);
                        // }
                        event::KeyReleased { code, .. } => {
                             if let Key::Escape = code {
                                 self.state_stack.push(StateType::Menu);
                                 println!("{:?}", self.state_stack);
                             }
                        //     if let Key::Space = code {
                        //         moving = !moving;
                        //     }
                        //     if let Key::P = code {

                        //     }
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
                                        if button.text.get_color().0 == Color::new_rgb(0, 255, 0).0 {
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
                    let (dx, dy) = {
                        let mult = 50. * dt;
                        (mult * a.move_dir.x, mult * a.move_dir.y)
                    };

                    if (dx, dy) != (0., 0.) {
                        let actor_bounds = a.shape.get_global_bounds();

                        let desired_pos = FloatRect::new(actor_bounds.left + dx,
                                                         actor_bounds.top + dy,
                                                         actor_bounds.width,
                                                         actor_bounds.height);
                        let mut ok_to_move = true;
                        for w in self.wagons.iter() {
                            for t in w.tiles.iter() {
                                for t in t.iter() {
                                    if !t.is_solid {
                                        continue;
                                    }
                                    for b in t.bounds.iter() {
                                        let b = if *b != None {
                                            b.unwrap()
                                        }
                                        else {
                                            continue;
                                        };
                                        if let Some(_) = desired_pos.intersects(
                                            &FloatRect::new(b.left as f32 + t.sprite.get_position().x,
                                                            b.top as f32 + t.sprite.get_position().y,
                                                            b.width as f32,
                                                            b.height as f32)) {
                                            ok_to_move = false;
                                            break;
                                        }
                                    }
                                }
                            }
                        }

                        if ok_to_move {
                            a.shape.move2f(dx, dy);
                        }

                        a.inside_wagon = false;
                        for w in self.wagons.iter() {
                            for t in w.tiles.iter() {
                                for t in t.iter() {
                                    if !t.is_solid && t.sprite.get_global_bounds().contains(a.shape.get_position()) { 
                                        a.inside_wagon = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }


                //if bg1.get_position().x <= -(WINDOW_SIZE_X as f32) {
                //     bg1.move2f((WINDOW_SIZE_X * 2) as f32, 0.);
                // }
                // if bg2.get_position().x <= -(WINDOW_SIZE_X as f32) {
                //     bg2.move2f((WINDOW_SIZE_X * 2) as f32, 0.);
                // }

                // if moving {
                //     if current_speed + accel < top_speed {
                //         current_speed += accel;
                //     }

                // } else if current_speed > 0. {
                //     current_speed -= accel * 2.;
                // }

                // bg1.move2f(dt * -current_speed, 0.);
                // bg2.move2f(dt * -current_speed, 0.);

                for a in self.actors.iter_mut() {
                    if !a.inside_wagon {
                        // add collision checking to this (refactor what is above into a checking function)
                        a.shape.move2f(dt * -50., 0.);
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

                //window.draw(&bg1);
//                window.draw(&bg2);

                for a in self.actors.iter() {
                    if !a.inside_wagon {
                        self.window.draw(&a.shape);
                    }
                }

                for w in self.wagons.iter() {
                    self.window.draw(w);
                }

                for a in self.actors.iter() {
                    if a.inside_wagon {
                        self.window.draw(&a.shape);
                    }
                }
            },
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