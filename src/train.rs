use sfml::system::*;
use sfml::graphics::*;

use pathfinding::*;
use wagon::*;

use game_consts::*;

pub struct Train<'a> {
    pub wagons: Vec<Wagon<'a>>,
    pub moving: bool,
    pub current_speed: f32,
    pub top_speed: f32,
    pub accel: f32,
    pub pfgrid_in: PathfindingGrid,
    pub pfgrid_out: PathfindingGrid,
    pub pfgrid_all: PathfindingGrid,
    pub total_size: Vector2u,
}

impl<'a> Train<'a> {
    pub fn new() -> Self {
        Train {
            wagons: vec![],
            moving: false,
            current_speed: 0.,
            top_speed: 0.,
            accel: 0.,
            pfgrid_in: PathfindingGrid::new(),
            pfgrid_out: PathfindingGrid::new(),
            pfgrid_all: PathfindingGrid::new(),
            total_size: Vector2u::new(0, 0),
        }
    }

    pub fn init(&mut self, top_speed: f32, accel: f32) {
        self.top_speed = top_speed;
        self.accel = accel;
    }

    pub fn update(&mut self) {
        if self.moving {
            if self.current_speed + self.accel < self.top_speed {
                self.current_speed += self.accel;
            }
        } else if self.current_speed > 0. {
            self.current_speed -= self.accel / 2.;
        }
    }

    pub fn set_position2f(&mut self, x: f32, y: f32) {
        let origin = self.wagons.last().unwrap().get_origin();
        let difference = Vector2f::new((origin.x - x).abs(), (origin.y - y).abs());
        for w in self.wagons.iter_mut().rev() {
            w.move2f(difference.x, difference.y);
//            last_pos = w.tiles[0][0].sprite.get_position().x * TILE_SIZE_X as f32;
        }
    }

    pub fn rebuild_pfgrids(&mut self) {
        let mut total_width = 0;
        let mut max_height = 0;

        for wag in self.wagons.iter() {
            total_width += wag.tiles[0].len();
            total_width -= 1;

            if wag.tiles.len() > max_height {
                max_height = wag.tiles.len();
            }
        }

        total_width += 1;

        let pad = (2, 2, 2, 2);
        // 0: top
        // 1: bot
        // 2: left
        // 3: right
        self.pfgrid_all.grid = vec![vec![PathfindingTile{ walkable:true }; (max_height + pad.2 + pad.3) as usize]; (total_width + pad.0 + pad.1) as usize];
        self.pfgrid_all.padding = pad;

        self.pfgrid_in.grid = vec![vec![PathfindingTile{ walkable:false }; (max_height + pad.2 + pad.3) as usize]; (total_width + pad.0 + pad.1) as usize];
        self.pfgrid_in.padding = pad;

        let mut prev_train_width = 0;

        let mut door_idxs: Vec<[(usize, usize); 2]> = vec![];
        for wagon in self.wagons.iter().rev() {
            let this_wagon_height = wagon.tiles.len();
            for (i, t) in wagon.tiles.iter().enumerate() {
                for (j, t) in t.iter().enumerate() {
                    let (x, y) = (pad.2 + j + prev_train_width, pad.0 + i + (max_height - this_wagon_height) / 2);
                    self.pfgrid_in.grid[x][y].walkable = !t.is_solid;
                    if let TileType::Door(ref dir) = t.tile_type {
                        let curr = (x, y);
                        door_idxs.push(
                            match *dir {
                                Direction::North => [ curr, (x, y + 1) ],
                                Direction::South => [ curr, (x, y - 1) ],
                                Direction::West => [ curr, (x + 1, y) ],
                                Direction::East => [ curr, (x - 1, y) ],
                            });
                    }
                }
            }
            prev_train_width += wagon.tiles[0].len() - 1;
        }

        // do all stuff to pfgrid_in before this
        self.pfgrid_out = self.pfgrid_in.clone();

        for (i, pft) in self.pfgrid_out.grid.iter_mut().enumerate() {
            for (j, pft) in pft.iter_mut().enumerate() {
                pft.walkable = !pft.walkable;
            }
        }

        for idxs in door_idxs.iter() {
            self.pfgrid_out.grid[idxs[0].0][idxs[0].1].walkable = true;
            if self.pfgrid_in.grid[idxs[0].0][idxs[0].1].walkable == true {
                self.pfgrid_out.grid[idxs[1].0][idxs[1].1].walkable = true;
            }
        }

        self.total_size.x = total_width as u32;
        self.total_size.y = max_height as u32;
    }

    pub fn get_origin(&self) -> Vector2f {
        let first_wagon_height = self.wagons.last().unwrap().tiles.len();
        let first_tile_pos = self.wagons.last().unwrap().get_origin();
        let train_pos = Vector2f::new(first_tile_pos.x,
                                      first_tile_pos.y - ((self.total_size.y as f32 - first_wagon_height as f32) / 2.) * TILE_SIZE_Y as f32);
        train_pos
    }
}
