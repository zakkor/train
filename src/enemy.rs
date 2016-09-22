extern crate sfml;
use sfml::graphics::*;
use sfml::system::*;
use std::collections::VecDeque;

use wagon::*;
use game_consts::*;
use astar::*;
use pathfinding::Pathfinding;

pub struct Enemy<'a> {
    pub sprite: Sprite<'a>,
    pub inside_wagon: bool,
    pub move_seq: VecDeque<Vector2f>,
}

impl<'a> Enemy<'a> {
    pub fn new(texture: &'a Texture) -> Self {
        let mut sprite = Sprite::new().unwrap();
        //sprite.set_size2f(25., 25.);
        sprite.set_color(&Color::yellow());
        sprite.set_position2f(25. + TILE_SIZE_X as f32 * 6.,
                              25. + TILE_SIZE_Y as f32 * 2.);
        sprite.set_origin2f(25. / 2., 25. / 2.);
        sprite.set_texture(texture, true);

        Enemy {
            sprite: sprite,
            inside_wagon: false,
            move_seq: VecDeque::new(),
        }
    }
}

impl<'a> Pathfinding for Enemy<'a> {
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
