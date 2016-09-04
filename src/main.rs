#![allow(dead_code)]
#![allow(unused_imports)]

extern crate sfml;
extern crate rand;

use rand::Rng;

use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;

// mod platform; use platform::*;
mod state_stack;
use state_stack::*;
mod resource_manager;
use resource_manager::*;

mod particle_manager;
use particle_manager::*;

mod player;
use player::*;
mod menu;
use menu::*;

#[derive(Clone)]
struct Tile<'a> {
    sprite: Sprite<'a>,
    is_solid: bool,
    bounds: IntRect
}

impl<'a> Tile<'a> {
    fn new_with_texture(texture: &'a Texture) -> Self {
        Tile {
            sprite: Sprite::new_with_texture(texture).unwrap(),
            is_solid: false,
            bounds: IntRect::new(0, 0, 0, 0)
        }
    }

    fn new() -> Self {
        Tile {
            sprite: Sprite::new().unwrap(),
            is_solid: false,
            bounds: IntRect::new(0, 0, 0, 0)
        }
    }
}

struct Wagon<'a> {
    tiles: Vec<Vec<Tile<'a>>>,
    connected_to: [Option<&'a mut Wagon<'a>>; 2]
}

const TILE_SIZE_X: u32 = 64;
const TILE_SIZE_Y: u32 = 64;

const WINDOW_SIZE_X: u32 = 1600;
const WINDOW_SIZE_Y: u32 = 900;

impl<'a> Wagon<'a> {
    /// Creates a new wagon of size `size_x, size_y` and places all its tiles with the corner at (0,0).
    fn new(tex_man: &'a TextureManager, size_x: u32, size_y: u32) -> Self {
        if size_y % 2 == 0 {
            panic!("wagon height needs to be an odd number");
        }
        
        let mut tiles: Vec<Vec<Tile>> = vec![];
        for i in 0..(size_y + 2) {
            tiles.push(vec![]);
            for j in 0..(size_x + 2) {
                let mut tile = Tile::new();
                tile.sprite.set_position2f((j * TILE_SIZE_X) as f32, (i * TILE_SIZE_Y) as f32);
                tile.is_solid = true;

                if (i, j) == (0, 0) {
                    tile.sprite.set_texture(tex_man.get(TextureId::CornerTopLeft), true);
                }
                else if (i, j) == (size_y + 1, 0) {
                    tile.sprite.set_texture(tex_man.get(TextureId::CornerBottomLeft), true);
                }
                else if (i, j) == (0, size_x + 1) {
                    tile.sprite.set_texture(tex_man.get(TextureId::CornerTopRight), true);
                }
                else if (i, j) == (size_y + 1, size_x + 1) {
                    tile.sprite.set_texture(tex_man.get(TextureId::CornerBottomRight), true);
                }
                else if j == 0 {
                    tile.sprite.set_texture(tex_man.get(TextureId::WallLeft), true);
                }
                else if j == size_x + 1 {
                    tile.sprite.set_texture(tex_man.get(TextureId::WallRight), true);
                }
                else if i == 0 {
                    tile.sprite.set_texture(tex_man.get(TextureId::WallTop), true);
                    tile.bounds = IntRect::new(0, 58, 64, 6);
                }
                else if i == size_y + 1 {
                    tile.sprite.set_texture(tex_man.get(TextureId::WallBottom), true);
                }
                else {
                    tile.sprite.set_texture(tex_man.get(TextureId::Floor), true);
                    tile.is_solid = false;
                }
                tiles[i as usize].push(tile);
            }
        }
        
        Wagon {
            tiles: tiles,
            connected_to: [None, None]
        }
    }

    fn set_position2f(&mut self, dest_x: f32, dest_y: f32) {
        for i in 0..(self.tiles.len()) {
            for j in 0..(self.tiles[i].len()) {
                let current_x = self.tiles[i][j].sprite.get_position().x;
                let current_y = self.tiles[i][j].sprite.get_position().y;
                self.tiles[i][j].sprite.set_position2f(current_x + dest_x, current_y + dest_y);
            }
        }

        if let Some(ref mut x) = self.connected_to[0] {
            x.set_position2f(dest_x, dest_y);
        }
    }

    /// Connects wagon `other` to the *left* side of wagon `self`.
    fn connect(& mut self, other: &mut Wagon<'a>, tex_man: &'a TextureManager) {
        let self_height = self.tiles.len();
        let self_height_half = self_height / 2;
        let other_height = other.tiles.len();
        let other_width = other.tiles[0].len();
        let other_height_half = other_height / 2;
        
        self.tiles[self_height_half - 1][0].sprite.set_texture(tex_man.get(TextureId::ConnectorTop), true);
        self.tiles[self_height_half][0].sprite.set_texture(tex_man.get(TextureId::Floor), true);
        self.tiles[self_height_half + 1][0].sprite.set_texture(tex_man.get(TextureId::ConnectorBottom), true);
        

        other.tiles[other_height_half - 1][other_width -1].sprite.set_texture(tex_man.get(TextureId::WallConnectedTop), true);
        other.tiles[other_height_half][other_width -1] = Tile::new();
        other.tiles[other_height_half + 1][other_width -1].sprite.set_texture(tex_man.get(TextureId::WallConnectedBottom), true);
        other.set_position2f((self.tiles[0][0].sprite.get_position().x - ((other_width - 1) * TILE_SIZE_X as usize) as f32) as f32, self.tiles[0][0].sprite.get_position().y + ((self_height_half - other_height_half) * TILE_SIZE_Y as usize) as f32);
    }
}

impl<'a> Drawable for Wagon<'a> {
    fn draw<RT: RenderTarget>(&self, render_target: &mut RT, render_states: &mut RenderStates) {
        for i in 0..(self.tiles.len()) {
            for j in 0..(self.tiles[i].len()) {
                render_target.draw(&self.tiles[i][j].sprite);
                if self.tiles[i][j].is_solid {
                    let mut shape = RectangleShape::new().unwrap();
                    shape.set_fill_color(&Color::new_rgba(255, 0, 0, 100));
                    let bounds = self.tiles[i][j].bounds;
                    shape.set_size2f(bounds.width as f32, bounds.height as f32);
                    shape.set_position2f(self.tiles[i][j].sprite.get_position().x + bounds.left as f32,
                                         self.tiles[i][j].sprite.get_position().y + bounds.top as f32);
                    
                    render_target.draw(&shape);
                }
            } 
        }
    }
}


fn main() {
    // Create the window of the application
    let mut window = RenderWindow::new(VideoMode::new_init(WINDOW_SIZE_X, WINDOW_SIZE_Y, 32),
                                       "Train",
                                       window_style::CLOSE,
                                       &ContextSettings::default())
        .unwrap();
    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);

    let mut font_manager = FontManager::new();
    font_manager.load(FontId::Arial, "res/arial.ttf");
    font_manager.load(FontId::Joystix, "res/joystix-mono.ttf");

    let mut view = window.get_default_view();
    //view.set_center2f(0.5, 0.5);
    window.set_view(&view);



    // let mut game_over_text = Text::new().unwrap();
    // game_over_text.set_font(font_manager.get(FontId::Arial));
    // game_over_text.set_position(&Vector2f::new(1280. / 2. - 175., 250.));
    // game_over_text.set_color(&Color::white());
    // game_over_text.set_character_size(60);
    // game_over_text.set_string("GAME OVER!");


    let mut tex_man = TextureManager::new();
    tex_man.load(TextureId::Floor, "res/floor.png");
    tex_man.load(TextureId::WallTop, "res/wall_top.png");
    tex_man.load(TextureId::WallBottom, "res/wall_bottom.png");
    tex_man.load(TextureId::WallLeft, "res/wall_left.png");
    tex_man.load(TextureId::WallRight, "res/wall_right.png");
    tex_man.load(TextureId::CornerTopLeft, "res/corner_topleft.png");
    tex_man.load(TextureId::CornerTopRight, "res/corner_topright.png");
    tex_man.load(TextureId::CornerBottomLeft, "res/corner_bottomleft.png");
    tex_man.load(TextureId::CornerBottomRight, "res/corner_bottomright.png");
    tex_man.load(TextureId::ConnectorTop, "res/connector_top.png");
    tex_man.load(TextureId::ConnectorBottom, "res/connector_bottom.png");
    tex_man.load(TextureId::WallConnectedTop, "res/wall_connected_top.png");
    tex_man.load(TextureId::WallConnectedBottom, "res/wall_connected_bottom.png");
    tex_man.load(TextureId::Background, "res/bg.png");

    let mut player = Player::new();


    let mut state_stack = StateStack::new();
    state_stack.push(StateType::Playing);

    let mut particle_manager = ParticleManager::new();


    // // view
    // let mut view = View::new_init(&Vector2f::new(1280. / 2., 720. / 2.),
    //                               &Vector2f::new(1280., 720.))
    //     .unwrap();
    // window.set_view(&view);

    
    // delta time
    let mut clock = Clock::new();

    let mut wagon1 = Wagon::new(&tex_man, 5, 5);
    let mut wagon2 = Wagon::new(&tex_man, 5, 5);
    wagon1.set_position2f(750., 225.);    
    wagon1.connect(&mut wagon2, &tex_man);

    let mut bg1 = Sprite::new_with_texture(&tex_man.get(TextureId::Background)).unwrap();
    let mut bg2 = bg1.clone();
    bg2.set_position2f(1600., 0.);


    // menu
    let mut menu = Menu {
        buttons: vec![Button::new(font_manager.get(FontId::Joystix),
                                  ButtonType::Resume,
                                  &Vector2f::new(150., 180.)),
                      Button::new(font_manager.get(FontId::Joystix),
                                  ButtonType::Quit,
                                  &Vector2f::new(150., 180. + 80.))],
    };

    let mut mouse_pos_old = Vector2f::new(WINDOW_SIZE_X as f32 / 2., WINDOW_SIZE_Y as f32 / 2.);

    // TODO: move to Train
    let mut current_speed = 0.;
    let top_speed = 1000.;
    let accel = 0.5;

    let mut moving = false;

    while window.is_open() {
        // ___________________ EVENTS_BEGIN ______________//
        {
            for event in window.events() {
                match *state_stack.top().unwrap() {
                    StateType::Playing => {
                        if MouseButton::Middle.is_pressed() {
                            let mouse_pos = window.map_pixel_to_coords_current_view(&window.get_mouse_position());
                            let move_factor = Vector2f::new(mouse_pos_old.x - mouse_pos.x,
                                                            mouse_pos_old.y - mouse_pos.y);
                            
                            view.move_(&move_factor);
                            window.set_view(&view);
                        }
                        mouse_pos_old = window.map_pixel_to_coords_current_view(&window.get_mouse_position());


                        
                        match event {
                            event::Closed => window.close(),
                            event::MouseMoved { x, .. } => {}
                            event::MouseButtonPressed { button, .. } => {
                                match button {
                                    MouseButton::Left => {}
                                    MouseButton::Right => {}
                                    
                                    _ => {}
                                }
                            }
                            event::MouseWheelMoved { delta, .. } => {
                                let zoom_factor = match delta < 0 {
                                    true => 1.1,
                                    false => 0.9
                                };

                                view.zoom(zoom_factor);
                                window.set_view(&view);
                            }
                            event::KeyReleased { code, .. } => {
                                if let Key::Escape = code {
                                    state_stack.push(StateType::Menu);
                                    println!("{:?}", state_stack);
                                }
                                if let Key::Space = code {
                                    moving = !moving;
                                }
                            }
                            _ => {
                                // do nothing
                            }
                        }
                    }
                    StateType::Menu => {
                        match event {
                            event::KeyReleased { code, .. } => {
                                match code {
                                    Key::Escape => {
                                        state_stack.pop();
                                        println!("{:?}", state_stack);
                                    }
                                    _ => {}
                                }
                            }
                            event::MouseMoved { x, y, .. } => {
                                for button in &mut menu.buttons {
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
                                        for button in &menu.buttons {
                                            // TODO: add hover
                                            if true {
                                                match button.button_type {
                                                    ButtonType::Quit => {
                                                        window.close();
                                                    }
                                                    ButtonType::Resume => {
                                                        state_stack.pop();
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
                        match event {
                            event::Closed => {
                                window.close();
                            }
                            event::KeyReleased { code, .. } => {
                                match code {
                                    Key::R => {
                                        // reset the game
                                        state_stack.pop();

                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }

        }
        // ___________________ EVENTS_END ______________//

        let time = clock.restart();
        match *state_stack.top().unwrap() {
            StateType::Playing => {
                {
                    // ___________________ UPDATE_BEGIN ______________//
                    let dt = time.as_seconds();
                    let (dx, dy) = {
                        let mult = 150. * dt;
                        if Key::W.is_pressed() {
                            (0., -mult)
                        } else if Key::A.is_pressed() {
                            (-mult, 0.)
                        } else if Key::S.is_pressed() {
                            (0., mult)
                        } else if Key::D.is_pressed() {
                            (mult, 0.)
                        } else {
                            (0., 0.)
                        }
                    };

                    if (dx, dy) != (0., 0.) {
                        let player_bounds = player.shape.get_global_bounds();
                        
                        let desired_pos = FloatRect::new(player_bounds.left + dx,
                                                         player_bounds.top + dy,
                                                         player_bounds.width,
                                                         player_bounds.height);
                        let mut ok_to_move = true;                    
                        for t in wagon2.tiles.iter() {
                            for t in t.iter() {
                                if !t.is_solid {
                                    continue;
                                }
                                if let Some(_) = desired_pos.intersects(
                                    &FloatRect::new(t.bounds.left as f32 + t.sprite.get_position().x,
                                                    t.bounds.top as f32 + t.sprite.get_position().y,
                                                    t.bounds.width as f32,
                                                    t.bounds.height as f32)) {
                                    ok_to_move = false;
                                    break;
                                }
                            }
                        }
                        
                        if ok_to_move {
                            player.shape.move2f(dx, dy);
                        }

                    }


                    if bg1.get_position().x <= -(WINDOW_SIZE_X as f32) {
                        bg1.move2f((WINDOW_SIZE_X * 2) as f32, 0.);
                    }
                    if bg2.get_position().x <= -(WINDOW_SIZE_X as f32) {
                        bg2.move2f((WINDOW_SIZE_X * 2) as f32, 0.);
                    }
                    
                    if moving {
                        if current_speed + accel < top_speed {
                            current_speed += accel;
                        }

                    } else if current_speed > 0. {
                        current_speed -= accel * 2.;
                    }

                    bg1.move2f(dt * -current_speed, 0.);
                    bg2.move2f(dt * -current_speed, 0.);
                    

                    // ___________________ UPDATE_END ________________//
                }

                {
                    // ___________________ RENDER_BEGIN  _____________//
                    // Set view
//                    window.set_view(&view);
                    // Clear the window
                    window.clear(&Color::black());

                    window.draw(&bg1);
                    window.draw(&bg2);
                    
                    window.draw(&wagon1);
                    window.draw(&wagon2);

                    window.draw(&player.shape);
//                    window.draw(&tile.sprite);

                    // ____________________ RENDER_END _____________//
                }
            }
            StateType::Menu => {
                // update
                {
                    // don't update anything for now
                }
                // render
                {
                    window.clear(&Color::black());

                    for button in &menu.buttons {
                        window.draw(&button.text);
                    }
                }
            }
            StateType::GameOver => {
                // update
                {
                    // don't update anything for now
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
