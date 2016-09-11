extern crate sfml;
use sfml::graphics::*;
use sfml::system::*;
use std::collections::VecDeque;

use wagon::*;

pub struct Actor<'a> {
    pub shape: RectangleShape<'a>,
    pub inside_wagon: bool,
    pub move_seq: VecDeque<Vector2f>,
}

impl<'a> Actor<'a> {
    pub fn new() -> Self {
        let mut shape = RectangleShape::new().unwrap();
        shape.set_size2f(25., 25.);
        shape.set_fill_color(&Color::red());
        shape.set_position2f(1280. / 2., 720. - 200.);
        shape.set_origin2f(25./2., 25./2.);

        Actor {
            shape: shape,
            inside_wagon: true,
            move_seq: VecDeque::new(),
        }
    }
}


pub trait Pathfinding {
    fn cast_line(starting_pos: &Vector2f, ending_pos: &Vector2f) -> bool {
        true
    }
}
