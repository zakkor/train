use sfml::system::*;
use game_consts::*;
use astar::*;
use std::vec::IntoIter;
use sfml::graphics::{FloatRect, Transformable};
use wagon::{Wagon, TileType};
use std::collections::VecDeque;
use std::thread;

#[derive(Copy, Clone)]
pub struct PathfindingTile {
    pub walkable: bool,
}

#[derive(Clone)]
pub struct PathfindingGrid {
    pub grid: Vec<Vec<PathfindingTile>>,
    pub padding: (usize, usize, usize, usize),
}

impl PathfindingGrid {
    pub fn new() -> Self {
        PathfindingGrid {
            grid: vec![],
            padding: (0, 0, 0, 0),
        }
    }
}

pub trait Walkable {
    fn is_walkable(&self, x: i32, y: i32) -> bool;
}

impl Walkable for Vec<Vec<PathfindingTile>> {
    fn is_walkable(&self, x: i32, y: i32) -> bool {
        self[x as usize][y as usize].walkable
    }
}

pub struct GridSearch<'a> {
    grid: &'a PathfindingGrid,
    start: (i32, i32),
    end: (i32, i32),
}

impl<'a> GridSearch<'a> {
    pub fn new(grid: &'a PathfindingGrid, start: (i32, i32), end: (i32, i32)) -> Self {
        GridSearch {
            grid: grid,
            start: start,
            end: end,
        }
    }
}

impl<'a> SearchProblem for GridSearch<'a> {
    type Node = (i32, i32);
    type Cost = i32;
    type Iter = IntoIter<((i32, i32), i32)>;
    fn start(&self) -> (i32, i32) {
        self.start
    }
    fn is_end(&self, p: &(i32, i32)) -> bool {
        *p == self.end
    }
    fn heuristic(&self, &(p_x, p_y): &(i32, i32)) -> i32 {
        let (s_x, s_y) = self.end;
        (s_x - p_x).abs() + (s_y - p_y).abs()
    }
    fn neighbors(&mut self, &(x, y): &(i32, i32)) -> IntoIter<((i32, i32), i32)> {
        let mut vec = vec![];
        for i in -1..1 + 1 {
            for k in -1..1 + 1 {
                if !(i == 0 && k == 0)
                    // fucking corners
                    && !(i == -1 && k == -1) && !(i == -1 && k == 1) &&
                    !(i == 1 && k == -1) && !(i == 1 && k == 1)
                    && x + i >= 0 && y + k >= 0 &&
                    x + i < self.grid.grid.len() as i32 &&
                    y + k < self.grid.grid[0].len() as i32 &&
                    self.grid.grid.is_walkable(x + i, y + k) {
                        vec.push(((x + i, y + k), 1));
                    }
            }
        }
        vec.into_iter()
    }
}

pub trait Pathfinding {
    fn clear_steps(&mut self);
    fn add_step(&mut self, step: Vector2f);
    fn get_pos(&self) -> Vector2f;
    fn steps_are_empty(&self) -> bool;
    fn get_first_step(&self) -> &Vector2f;
    fn pop_first_step(&mut self);
    fn get_collision_bounds(&self) -> FloatRect;
    fn move2f(&mut self, x: f32, y: f32);
    fn set_inside_wagon(&mut self, inside: bool);


    fn set_path(&mut self, path: &mut VecDeque<(i32, i32)>, train_pos: Vector2f) {
        self.clear_steps();

        let mut path = path.iter();
        path.next();

        for step in path {
            self.add_step(Vector2f::new((step.0 as f32 + (train_pos.y - 2. * 64.) / TILE_SIZE_X as f32) * TILE_SIZE_X as f32 +
                                        TILE_SIZE_X as f32 / 2.,
                                        (step.1 as f32 + (train_pos.y - 2. * 64.) / TILE_SIZE_Y as f32) * TILE_SIZE_Y as f32 +
                                        TILE_SIZE_Y as f32 / 2.));
        }
    }

    fn number_of_steps_to(&self, grid: &PathfindingGrid, train_pos: &Vector2f, click_pos: Vector2f) -> Option<usize> {
        let start = (self.get_pos().x as i32 / TILE_SIZE_X as i32
                     - (train_pos.x - grid.padding.2 as f32 * TILE_SIZE_X as f32) as i32 / TILE_SIZE_X as i32,

                     self.get_pos().y as i32 / TILE_SIZE_Y as i32
                     - (train_pos.y - grid.padding.0 as f32 * TILE_SIZE_X as f32) as i32 / TILE_SIZE_Y as i32);

        let end = (click_pos.x as i32 / TILE_SIZE_X as i32 - (train_pos.x - 2. * 64.) as i32 / TILE_SIZE_X as i32,
                   click_pos.y as i32 / TILE_SIZE_Y as i32 - (train_pos.y - 2. * 64.) as i32 / TILE_SIZE_Y as i32);

        let mut ts = GridSearch::new(grid, start, end);

        if let Some(path) = astar(&mut ts) {
            let mut path = path.iter();
            path.next();

            Some(path.len())
        } else {
            None
        }
    }

    fn check_collision(desired_pos: &FloatRect, wagons: &Vec<Wagon>) -> bool {
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

    fn update_movement(&mut self, wagons: &Vec<Wagon>, dt: f32) {
        if self.steps_are_empty() {
            return;
        }

        let current_pos = self.get_pos();

        let dest = self.get_first_step().clone();
        if (dest.x - current_pos.x).abs() < 4. && (dest.y - current_pos.y).abs() < 4. {
            self.pop_first_step();
            return;
        }

        let mut move_dir = Vector2f::new(dest.x - current_pos.x, dest.y - current_pos.y);
        let vec_len = (move_dir.x.powi(2) + move_dir.y.powi(2)).sqrt().abs();

        move_dir.x = move_dir.x / vec_len;
        move_dir.y = move_dir.y / vec_len;

        let (dx, dy) = {
            // how much to move per frame
            let mult = 300. * dt;
            (mult * move_dir.x, mult * move_dir.y)
        };

        if (dx, dy) != (0., 0.) {
            let actor_bounds = self.get_collision_bounds();
            let desired_pos = FloatRect::new(actor_bounds.left + dx,
                                             actor_bounds.top + dy,
                                             actor_bounds.width,
                                             actor_bounds.height);

            if Self::check_collision(&desired_pos, &wagons) {
                self.move2f(dx, dy);

                // check and mark if player is inside or outside the train after a succesful move
                self.set_inside_wagon(false);
                for w in wagons.iter() {
                    for t in w.tiles.iter() {
                        for t in t.iter() {
                            if !t.is_solid &&
                                t.sprite.get_global_bounds().contains(self.get_pos()) {
                                    if let TileType::Door(_) = t.tile_type {
                                        continue;
                                    } else {
                                        self.set_inside_wagon(true);
                                        break;
                                    }
                                }
                        }
                    }
                }
            }
        }
    }
}


pub fn compute_path(start: Vector2f, grid: PathfindingGrid, train_pos: Vector2f, click_pos: Vector2f) -> Option<VecDeque<(i32, i32)>> {
    let start = (start.x as i32 / TILE_SIZE_X as i32
                 - (train_pos.x - grid.padding.2 as f32 * TILE_SIZE_X as f32) as i32 / TILE_SIZE_X as i32,
                 start.y as i32 / TILE_SIZE_Y as i32
                 - (train_pos.y - grid.padding.0 as f32 * TILE_SIZE_X as f32) as i32 / TILE_SIZE_Y as i32);

    let end = (click_pos.x as i32 / TILE_SIZE_X as i32 - (train_pos.x - 2. * 64.) as i32 / TILE_SIZE_X as i32,
               click_pos.y as i32 / TILE_SIZE_Y as i32 - (train_pos.y - 2. * 64.) as i32 / TILE_SIZE_Y as i32);

    //        self.clear_steps();

    let mut ts = GridSearch::new(&grid, start, end);

    astar(&mut ts)
}
