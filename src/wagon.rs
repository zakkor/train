
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
        let origin = self.get_origin();

        for tls in self.tiles.iter_mut() {
            for t in tls.iter_mut() {
                let current_pos = t.sprite.get_position();
                t.sprite.set_position2f(x + current_pos.x - origin.x, y + current_pos.y - origin.y);
            }
        }
    }

    pub fn move2f(&mut self, x: f32, y: f32) {
        for tls in self.tiles.iter_mut() {
            for t in tls.iter_mut() {
                t.sprite.move2f(x, y);
            }
        }
    }

    pub fn rotate(&mut self, angle: f32) {
        let angle_rad = angle * ::std::f64::consts::PI as f32 / 180.;
        let orig = self.get_origin() + self.get_middle();

        let formula_rot = |pos: Vector2f| {
            let mut point = Vector2f::new(0., 0.);
            point.x = pos.x * angle_rad.cos() - pos.y * angle_rad.sin();
            point.y = pos.x * angle_rad.sin() + pos.y * angle_rad.cos();
            point
        };

        for t in self.tiles.iter_mut() {
            for t in t.iter_mut() {
                t.sprite.move2f(-orig.x, -orig.y);
                let pos = t.sprite.get_position();

                let new = formula_rot(pos);

                t.sprite.set_position(&(orig + new));

                t.sprite.rotate(angle);
            }
        }
    }

    pub fn set_rotation(&mut self, angle: f32) {
        let angle_rad = angle * ::std::f64::consts::PI as f32 / 180.;
        let orig = self.get_origin() + self.get_middle();

        let formula_rot = |pos: Vector2f| {
            let mut point = Vector2f::new(0., 0.);
            point.x = pos.x * angle_rad.cos() - pos.y * angle_rad.sin();
            point.y = pos.x * angle_rad.sin() + pos.y * angle_rad.cos();
            point
        };

        for t in self.tiles.iter_mut() {
            for t in t.iter_mut() {
                t.sprite.move2f(-orig.x, -orig.y);
                let pos = t.sprite.get_position();

                let new = formula_rot(pos);

                t.sprite.set_position(&(orig + new));

                t.sprite.rotate(angle);
            }
        }
    }

    pub fn get_middle(&self) -> Vector2f {
        let width = self.tiles[0].len() as f32 * TILE_SIZE_X as f32 / 2.;
        let height = self.tiles.len() as f32 * TILE_SIZE_Y as f32 / 2.;
        Vector2f::new(width, height)
    }

    pub fn get_origin(&self) -> Vector2f {
        self.tiles[0][0].sprite.get_position().clone()
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

        // note: don't make this "better"
        let y_offset = if self_height_half > other_height_half {
            (self_height_half - other_height_half) as i32
        } else {
            -((other_height_half - self_height_half) as i32)
        } * TILE_SIZE_Y as i32;

        other.set_position2f((self.tiles[0][0].sprite.get_position().x - ((other_width - 1) * TILE_SIZE_X as usize) as f32) as f32,
                             self.tiles[0][0].sprite.get_position().y + y_offset as f32);
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
