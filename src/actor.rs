extern crate sfml;
use sfml::graphics::*;
use sfml::system::*;
use std::collections::VecDeque;

use wagon::*;
use game_consts::*;
use astar::*;
use pathfinding::*;

pub struct Actor<'a> {
    pub sprite: Sprite<'a>,
    pub inside_wagon: bool,
    pub move_seq: VecDeque<Vector2f>,
    pub rotation: f32,
}

impl<'a> Actor<'a> {
    pub fn new(texture: &'a Texture) -> Self {
        let mut sprite = Sprite::new().unwrap();
//        sprite.set_size2f(25., 25.);
        sprite.set_position2f(1280. / 2. - 400., 215.);
        sprite.set_origin2f(16., 16.);
        sprite.set_texture(texture, true);

        Actor {
            sprite: sprite,
            inside_wagon: true,
            move_seq: VecDeque::new(),
            rotation: 0.,
        }
    }
}

impl<'a> Pathfinding for Actor<'a> {
    fn clear_steps(&mut self) {
        self.move_seq.clear();
    }
    fn add_step(&mut self, step: Vector2f) {
        self.move_seq.push_back(step);
    }
    fn get_pos(&self) -> Vector2f {
        self.sprite.get_position()
    }
    fn steps_are_empty(&self) -> bool {
        self.move_seq.is_empty()
    }
    fn get_first_step(&self) -> &Vector2f {
        self.move_seq.front().unwrap()
    }
    fn pop_first_step(&mut self) {
        self.move_seq.pop_front();
    }
    fn get_collision_bounds(&self) -> FloatRect {
        self.sprite.get_global_bounds()
    }
    fn move2f(&mut self, x: f32, y: f32) {
        self.sprite.move2f(x, y);
    }
    fn set_inside_wagon(&mut self, inside: bool) {
        self.inside_wagon = inside;
    }
}
