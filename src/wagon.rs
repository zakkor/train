
extern crate sfml;
use sfml::graphics::*;
use sfml::system::*;
use sfml::window::*;
use sfml::audio::*;

use resource_manager::*;
use game_consts::*;
use astar::*;

use std::vec::IntoIter;
use std::collections::VecDeque;
use pathfinding::{PathfindingGrid, PathfindingTile};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    West,
    East
}

#[derive(Clone, PartialEq)]
pub enum TileType {
    WallAndFloor,
    Door(Direction),
    Window(Direction),
}

#[derive(Clone)]
pub struct Tile<'a> {
    pub sprite: Sprite<'a>,
    pub is_solid: bool,
    pub bounds: [Option<FloatRect>; 2],
    pub tile_type: TileType,
}

impl<'a> Tile<'a> {
    pub fn new() -> Self {
        Tile {
            sprite: Sprite::new().unwrap(),
            is_solid: false,
            bounds: [None; 2],
            tile_type: TileType::WallAndFloor,
        }
    }

    pub fn new_with_texture(texture: &'a Texture) -> Self {
        let mut new_tile = Tile::new();
        new_tile.sprite.set_texture(texture, false);
        new_tile
    }
}


pub struct Wagon<'a> {
    pub tiles: Vec<Vec<Tile<'a>>>,
    pub connected_to: [Option<&'a mut Wagon<'a>>; 2],
}


impl<'a> Wagon<'a> {
    //! Creates a new wagon of size `size_x, size_y` and places all its tiles with the corner at (0,0).
    pub fn new(tex_man: &'a TextureManager, size_x: u32, size_y: u32) -> Self {
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
                    tile.bounds[0] = Some(FloatRect::new(58., 58., 6., 6.));
                } else if (i, j) == (size_y + 1, 0) {
                    tile.sprite.set_texture(tex_man.get(TextureId::CornerBottomLeft), true);

                    tile.bounds[0] = Some(FloatRect::new(58., 0., 6., 6.));
                } else if (i, j) == (0, size_x + 1) {
                    tile.sprite.set_texture(tex_man.get(TextureId::CornerTopRight), true);
                    tile.bounds[0] = Some(FloatRect::new(0., 58., 6., 6.));
                } else if (i, j) == (size_y + 1, size_x + 1) {
                    tile.sprite.set_texture(tex_man.get(TextureId::CornerBottomRight), true);
                    tile.bounds[0] = Some(FloatRect::new(0., 0., 6., 6.));
                } else if j == 0 {
                    tile.sprite.set_texture(tex_man.get(TextureId::WallLeft), true);
                    tile.bounds[0] = Some(FloatRect::new(58., 0., 6., 64.));
                } else if j == size_x + 1 {
                    tile.sprite.set_texture(tex_man.get(TextureId::WallRight), true);
                    tile.bounds[0] = Some(FloatRect::new(0., 0., 6., 64.));
                } else if i == 0 && j == size_x / 2 {
                    tile.tile_type = TileType::Door(Direction::North);
                    tile.sprite.set_texture(tex_man.get(TextureId::DoorClosed(Direction::North)), true);
                } else if i == 0 {
                    tile.sprite.set_texture(tex_man.get(TextureId::WallTop), true);
                    tile.bounds[0] = Some(FloatRect::new(0., 58., 64., 6.));
                } else if i == size_y + 1 && j == size_x / 2 {
                    tile.tile_type = TileType::Door(Direction::South);
                    tile.sprite.set_texture(tex_man.get(TextureId::DoorClosed(Direction::South)), true);
                } else if i == size_y + 1 {
                    tile.sprite.set_texture(tex_man.get(TextureId::WallBottom), true);
                    tile.bounds[0] = Some(FloatRect::new(0., 0., 64., 6.));
                } else {
                    tile.sprite.set_texture(tex_man.get(TextureId::Floor), true);
                    tile.is_solid = false;
                }
                tiles[i as usize].push(tile);
            }
        }

        Wagon {
            tiles: tiles,
            connected_to: [None, None],
        }
    }

    pub fn set_position2f(&mut self, x: f32, y: f32) {
        let origin = self.tiles[0][0].sprite.get_position();
        let difference = Vector2f::new((x - origin.x).abs(), (y - origin.y).abs());

        for tls in self.tiles.iter_mut() {
            for t in tls.iter_mut() {
                t.sprite.move_(&difference);
            }
        }

        //     // if let Some(ref mut x) = self.connected_to[0] {
        //     //     x.set_position2f(dest_x, dest_y);
        //     // }
    }

    pub fn move2f(&mut self, x: f32, y: f32) {
        for tls in self.tiles.iter_mut() {
            for t in tls.iter_mut() {
                t.sprite.move2f(x, y);
            }
        }
    }

    /// Connects wagon `other` to the *left* side of wagon `self`.
    pub fn connect(&mut self, other: &mut Wagon<'a>, tex_man: &'a TextureManager) {
        let self_height = self.tiles.len();
        let self_height_half = self_height / 2;
        let other_height = other.tiles.len();
        let other_width = other.tiles[0].len();
        let other_height_half = other_height / 2;

        self.tiles[self_height_half - 1][0]
            .sprite
            .set_texture(tex_man.get(TextureId::ConnectorTop), true);
        self.tiles[self_height_half - 1][0].bounds[1] = Some(FloatRect::new(0., 58., 64., 6.));

        self.tiles[self_height_half][0].sprite.set_texture(tex_man.get(TextureId::Floor), true);
        self.tiles[self_height_half][0].is_solid = false;

        self.tiles[self_height_half + 1][0]
            .sprite
            .set_texture(tex_man.get(TextureId::ConnectorBottom), true);
        self.tiles[self_height_half + 1][0].bounds[1] = Some(FloatRect::new(0., 0., 64., 6.));

        other.tiles[other_height_half - 1][other_width - 1]
            .sprite
            .set_texture(tex_man.get(TextureId::WallConnectedTop), true);
        other.tiles[other_height_half][other_width - 1] = {
            let mut tile = Tile::new();
            tile.is_solid = true;
            tile
        };
        other.tiles[other_height_half + 1][other_width - 1]
            .sprite
            .set_texture(tex_man.get(TextureId::WallConnectedBottom), true);

        let y_offset = if self_height_half > other_height_half {
            (self_height_half - other_height_half) as i32
        } else {
            -((other_height_half - self_height_half) as i32)
        } * TILE_SIZE_Y as i32;

//        other.set_position2f((self.tiles[0][0].sprite.get_position().x - ((other_width - 1) * TILE_SIZE_X as usize) as f32) as f32,
//                             self.tiles[0][0].sprite.get_position().y + y_offset as f32);

//        self.connected_to[0] = Some(other);
//        other.connected_to[1] = Some(self);
    }
}

impl<'a> Drawable for Wagon<'a> {
    fn draw<RT: RenderTarget>(&self, render_target: &mut RT, render_states: &mut RenderStates) {
        for i in 0..(self.tiles.len()) {
            for j in 0..(self.tiles[i].len()) {
                render_target.draw(&self.tiles[i][j].sprite);
                // if self.tiles[i][j].is_solid {
                //     let bounds = self.tiles[i][j].bounds;
                //     for b in bounds.iter() {
                //         let b = if *b != None {
                //             b.unwrap()
                //         }
                //         else {
                //             continue;
                //         };
                //         let mut shape = RectangleShape::new().unwrap();
                //         shape.set_fill_color(&Color::new_rgba(0, 0, 255, 100));
                //         shape.set_size2f(b.width as f32, b.height as f32);
                //         shape.set_position2f(self.tiles[i][j].sprite.get_position().x + b.left as f32,
                //                              self.tiles[i][j].sprite.get_position().y + b.top as f32);

                //         render_target.draw(&shape);
                //     }
                // }
            }
        }
    }
}

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
        let difference = (self.wagons.last().unwrap().tiles[0][0].sprite.get_position().x - x).abs();
        for w in self.wagons.iter_mut().rev() {
            w.move2f(difference, y);
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
        let first_tile_pos = self.wagons.last().unwrap().tiles[0][0].sprite.get_position();
        let train_pos = Vector2f::new(first_tile_pos.x,
                                      first_tile_pos.y - ((self.total_size.y as f32 - first_wagon_height as f32) / 2.) * TILE_SIZE_Y as f32);
        train_pos
    }
}
