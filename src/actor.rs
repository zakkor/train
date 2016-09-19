extern crate sfml;
use sfml::graphics::*;
use sfml::system::*;
use std::collections::VecDeque;

use wagon::*;
use game_consts::*;
use astar::*;

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
        shape.set_position2f(1280. / 2. - 100., 720. - 400.);
        shape.set_origin2f(25. / 2., 25. / 2.);

        Actor {
            shape: shape,
            inside_wagon: true,
            move_seq: VecDeque::new(),
        }
    }

    pub fn check_collision(&self, desired_pos: &FloatRect, wagons: &Vec<Wagon>) -> bool {
        let mut ok_to_move = true;
        for w in wagons.iter() {
            for t in w.tiles.iter() {
                for t in t.iter() {
                    if !t.is_solid {
                        continue;
                    }
                    for b in t.bounds.iter() {
                        if let Some(b) = *b {
                            if let Some(_) = desired_pos.intersects(
                                &FloatRect::new(b.left + t.sprite.get_position().x,
                                                b.top + t.sprite.get_position().y,
                                                b.width,
                                                b.height)) {
                                ok_to_move = false;
                                break;
                            }
                        }
                    }
                }
            }
        }

        ok_to_move
    }

    pub fn update_movement(&mut self, wagons: &Vec<Wagon>, dt: f32) {
        if self.move_seq.is_empty() {
            return;
        }

        let current_pos = self.shape.get_position();

        let dest = self.move_seq.front().unwrap().clone();
        if (dest.x - current_pos.x).abs() < 1. && (dest.y - current_pos.y).abs() < 1. {
            self.move_seq.pop_front();
            return;
        }

        let mut move_dir = Vector2f::new(dest.x - current_pos.x, dest.y - current_pos.y);
        let vec_len = (move_dir.x.powi(2) + move_dir.y.powi(2)).sqrt().abs();

        move_dir.x = move_dir.x / vec_len;
        move_dir.y = move_dir.y / vec_len;

        let (dx, dy) = {
            // how much to move per frame
            let mult = 100. * dt;
            (mult * move_dir.x, mult * move_dir.y)
        };

        if (dx, dy) != (0., 0.) {
            let actor_bounds = self.shape.get_global_bounds();
            let desired_pos = FloatRect::new(actor_bounds.left + dx,
                                             actor_bounds.top + dy,
                                             actor_bounds.width,
                                             actor_bounds.height);

            if self.check_collision(&desired_pos, &wagons) {
                self.shape.move2f(dx, dy);

                // check and mark if player is inside or outside the train after a succesful move
                self.inside_wagon = false;
                for w in wagons.iter() {
                    for t in w.tiles.iter() {
                        for t in t.iter() {
                            if !t.is_solid &&
                               t.sprite.get_global_bounds().contains(self.shape.get_position()) {
                                self.inside_wagon = true;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub trait Pathfinding {
    fn set_path(&mut self, grid: &PathfindingGrid, click_pos: Vector2f) -> bool;
}

impl<'a> Pathfinding for Actor<'a> {
    fn set_path(&mut self, grid: &PathfindingGrid, click_pos: Vector2f) -> bool {
        let start = (self.shape.get_position().x as i32 / TILE_SIZE_X as i32,
                     self.shape.get_position().y as i32 / TILE_SIZE_Y as i32);

        let end = (click_pos.x as i32 / TILE_SIZE_X as i32,
                   click_pos.y as i32 / TILE_SIZE_Y as i32);

        self.move_seq.clear();

        let mut ts = TrainSearch::new(grid, start, end);

        if let Some(path) = astar(&mut ts) {
            for step in path.iter() {
                self.move_seq.push_back(Vector2f::new(step.0 as f32 * TILE_SIZE_X as f32 +
                                                      TILE_SIZE_X as f32 / 2.,
                                                      step.1 as f32 * TILE_SIZE_Y as f32 +
                                                      TILE_SIZE_Y as f32 / 2.));
            }
            true
        } else {
            false
        }
    }
}
