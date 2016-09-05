extern crate sfml;
use sfml::graphics::*;
use sfml::system::*;

use wagon::*;

//pub type ActorId = i32;

//static mut NUMBER_OF_ACTORS: ActorId = 0;

pub struct Player<'a> {
    pub shape: RectangleShape<'a>,
    pub inside_wagon: bool,
    //    pub id: ActorId,
    pub move_dir: Vector2f,
}

impl<'a> Player<'a> {
    pub fn new() -> Self {
        let mut shape = RectangleShape::new().unwrap();
        shape.set_size(&Vector2f::new(25., 25.));
        shape.set_fill_color(&Color::red());
        shape.set_position(&Vector2f::new(1280. / 2., 720. - 200.));
        shape.set_origin(&Vector2f::new(25./2., 25./2.));

       // NUMBER_OF_ACTORS += 1;
        
        Player {
            shape: shape,
            inside_wagon: true,
            //            id: NUMBER_OF_ACTORS,
            move_dir: Vector2f::new(0., 0.)
        }
    }
}


