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
use world::*;
use camera::*;

pub struct Game<'a> {
    resources: &'a Resources,
    window: RenderWindow,
    state_stack: StateStack,
    pm: ParticleManager<'a>,
    clock: Clock,
    train: Train<'a>,
    actors: Vec<Actor<'a>>,
    selected_actor: Option<usize>,
    menu: Menu<'a>,
    world: World<'a>,
    camera: Camera,
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

        let mut state_stack = StateStack::new();
        state_stack.push(StateType::Playing);

        Game {
            resources: resources,
            window: window,
            state_stack: state_stack,
            pm: ParticleManager::new(),
            clock: Clock::new(),
            train: Train::new(),
            actors: vec![],
            selected_actor: None,
            menu: Menu { buttons: vec![] },
            world: World { bgs: vec![] },
            camera: Camera::new()
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

        /*<test>*/
        self.train.wagons.push(Wagon::new(&self.resources.tm, 5, 7));
        self.train.wagons.push(Wagon::new(&self.resources.tm, 3, 7));

        self.train.wagons[0].set_position2f(300., 300.);
        /*</test>*/

        {
            let (a, b) = self.train.wagons.split_at_mut(1);
            a[0].connect(&mut b[0], &self.resources.tm);
        }

        self.actors.push(Actor::new());
    }

    fn process_events(&mut self) {
        for event in self.window.events() {
            match *self.state_stack.top().unwrap() {
                StateType::Playing => {
                    // Camera movement
                    if MouseButton::Middle.is_pressed() {
                        self.camera.move_by_mouse(&self.window.map_pixel_to_coords_current_view(&self.window.get_mouse_position()));

                        self.window.set_view(&self.camera.view);
                    }
                    self.camera.mouse_pos_old = self.window.map_pixel_to_coords_current_view(&self.window.get_mouse_position());

                    match event {
                        event::Closed => self.window.close(),
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
                        event::MouseWheelMoved { delta, .. } => {
                            self.camera.zoom(delta);
                            self.window.set_view(&self.camera.view);
                        }
                        event::KeyReleased { code, .. } => {
                            if let Key::Escape = code {
                                self.state_stack.push(StateType::Menu);
                                println!("{:?}", self.state_stack);
                            }
                            if let Key::Space = code {
                                self.train.moving = !self.train.moving;
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
                        for w in self.train.wagons.iter() {
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
                        for w in self.train.wagons.iter() {
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

                self.world.update();

                self.train.update();

                for bg in self.world.bgs.iter_mut() {
                    bg.move2f(dt * -self.train.current_speed, 0.);
                }

                for a in self.actors.iter_mut() {
                    if !a.inside_wagon {
                        // add collision checking to this (refactor what is above into a checking function)
                        a.shape.move2f(dt * -self.train.current_speed, 0.);
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

                for bg in self.world.bgs.iter_mut() {
                    self.window.draw(bg);
                }

                for a in self.actors.iter() {
                    if !a.inside_wagon {
                        self.window.draw(&a.shape);
                    }
                }

                for w in self.train.wagons.iter() {
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
