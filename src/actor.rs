extern crate sfml;
use sfml::graphics::*;
use sfml::system::*;

use wagon::*;

pub struct Actor<'a> {
    pub shape: RectangleShape<'a>,
    pub inside_wagon: bool,
    pub move_dir: Vector2f,
}

impl<'a> Actor<'a> {
    pub fn new() -> Self {
        let mut shape = RectangleShape::new().unwrap();
        shape.set_size(&Vector2f::new(25., 25.));
        shape.set_fill_color(&Color::red());
        shape.set_position(&Vector2f::new(1280. / 2., 720. - 200.));
        shape.set_origin(&Vector2f::new(25./2., 25./2.));

        Actor {
            shape: shape,
            inside_wagon: true,
            move_dir: Vector2f::new(0., 0.)
        }
    }
}
