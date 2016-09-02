extern crate sfml;
extern crate rand;

use rand::Rng;

use sfml::graphics::*; use sfml::system::*; use sfml::window::*;

//mod platform; use platform::*;
mod state_stack; use state_stack::*;
mod resource_manager; use resource_manager::*;
//mod score; use score::Score;
mod particle_manager; use particle_manager::*;
//mod util; use util::are_colors_equal;
mod player; use player::*;
mod menu; use menu::*;

fn main() {
    // Create the window of the application
    let mut window = RenderWindow::new(VideoMode::new_init(1280, 720, 32),
                                       "Train",
                                       window_style::CLOSE,
                                       &ContextSettings::default())
        .unwrap();
    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);

    let mut font_manager = FontManager::new();
    font_manager.load(FontIdentifiers::Arial, "res/arial.ttf");
    font_manager.load(FontIdentifiers::Joystix, "res/joystix-mono.ttf");



    // let mut game_over_text = Text::new().unwrap();
    // game_over_text.set_font(font_manager.get(FontIdentifiers::Arial));
    // game_over_text.set_position(&Vector2f::new(1280. / 2. - 175., 250.));
    // game_over_text.set_color(&Color::white());
    // game_over_text.set_character_size(60);
    // game_over_text.set_string("GAME OVER!");


    let mut texture_manager = TextureManager::new();


    let mut player = Player::new();


    let mut state_stack = StateStack::new();
    state_stack.push(StateType::Playing);

    let mut particle_manager = ParticleManager::new();

    // delta time
    let mut clock = Clock::new();

    // view
    let mut view = View::new_init(&Vector2f::new(1280./2., 720./2.), &Vector2f::new(1280., 720.)).unwrap();
    window.set_view(&view);

    // menu
    let mut menu = Menu {
        buttons: vec![Button::new(font_manager.get(FontIdentifiers::Joystix), ButtonType::Resume, &Vector2f::new(150., 180.)),
                      Button::new(font_manager.get(FontIdentifiers::Joystix), ButtonType::Quit, &Vector2f::new(150., 180.+ 80.))]
    };

    while window.is_open() {
        //___________________ EVENTS_BEGIN ______________//
        {
            for event in window.events() {
                match *state_stack.top().unwrap() {
                    StateType::Playing => {
                        match event {
                            event::Closed => window.close(),
                            event::MouseMoved { x, .. } => {
      
                            }
                            event::MouseButtonReleased { button, .. } => {
                                match button {
                                    MouseButton::Left => {},
                                    MouseButton::Right => {},
                                    _ => {}
                                }
                            }
                            event::KeyReleased { code, .. } => {
                                if let Key::Escape = code {
                                    state_stack.push(StateType::Menu);
                                    println!("{:?}", state_stack);
                                }
                                if let Key::Space = code {

                                }
                            }
                            _ => { /* do nothing */ }
                        }
                    },
                    StateType::Menu => {
                        match event {
                            event::KeyReleased { code, .. } => {
                                match code {
                                    Key::Escape => {
                                        state_stack.pop();
                                        println!("{:?}", state_stack);
                                    },
                                    _ => {}
                                }
                            },
                            event::MouseMoved { x, y, .. } => {
                                for button in &mut menu.buttons {
                                    let x = x as f32;
                                    let y = y as f32;
                                    if x > button.text.get_position().x
                                        && x < (button.text.get_position().x + button.text.get_local_bounds().width)
                                        && y > button.text.get_position().y
                                        && y < (button.text.get_position().y + button.text.get_local_bounds().height * 2.) { // <- *2. because Text bounding box is broken - SFML bug?
                                            button.text.set_color(&Color::green());
                                        }
                                    else {
                                        button.text.set_color(&Color::white());
                                    }
                                }
                            },
                            event::MouseButtonReleased { button, .. } => {
                                match button {
                                    MouseButton::Left => {
                                        for button in &menu.buttons {
                                            // TODO: add hover
                                            if true {
                                                match button.button_type {
                                                    ButtonType::Quit => {
                                                        window.close();
                                                    },
                                                    ButtonType::Resume => {
                                                        state_stack.pop();
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    _ => {}
                                }
                            },
                            _ => {}
                        }
                    },
                    StateType::GameOver => {
                        match event {
                            event::Closed => { window.close(); },
                            event::KeyReleased { code, .. } => {
                                match code {
                                    Key::R => {
                                        //reset the game
                                        state_stack.pop();

                                    },
                                    _ => {}
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
            
        }
        //___________________ EVENTS_END ______________//

        let time = clock.restart();
        match *state_stack.top().unwrap() {
            StateType::Playing => {
                {
                    //___________________ UPDATE_BEGIN ______________//
                    let dt = time.as_seconds();

                    // reset view
                    view.set_center(&Vector2f::new(1280./2., 720./2.));

                    //___________________ UPDATE_END ________________//
                }

                {
                    //___________________ RENDER_BEGIN  _____________//
                    // Set view
                    window.set_view(&view);
                    // Clear the window
                    window.clear(&Color::black());

                    //____________________ RENDER_END _____________//
                }
            },
            StateType::Menu => {
                // update
                {
                    /* don't update anything for now */
                }
                // render
                {
                    window.clear(&Color::black());

                    for button in &menu.buttons {
                        window.draw(&button.text);
                    }
                }
            },
            StateType::GameOver => {
                // update
                {
                    /* don't update anything for now */
                }
                // render
                {
                    window.clear(&Color::black());

                }
            }
        }
        window.display();
    }
}
