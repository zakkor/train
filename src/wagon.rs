
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

#[derive(Clone)]
pub struct Tile<'a> {
    pub sprite: Sprite<'a>,
    pub is_solid: bool,
    pub bounds: [Option<FloatRect>; 2]
}

impl<'a> Tile<'a> {
    pub fn new_with_texture(texture: &'a Texture) -> Self {
        Tile {
            sprite: Sprite::new_with_texture(texture).unwrap(),
            is_solid: false,
            bounds: [None; 2]
        }
    }

    pub fn new() -> Self {
        Tile {
            sprite: Sprite::new().unwrap(),
            is_solid: false,
            bounds: [None; 2]
        }
    }
}


pub struct Wagon<'a> {
    pub tiles: Vec<Vec<Tile<'a>>>,
    pub connected_to: [Option<&'a mut Wagon<'a>>; 2]
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
                }
                else if (i, j) == (size_y + 1, 0) {
                    tile.sprite.set_texture(tex_man.get(TextureId::CornerBottomLeft), true);

                    tile.bounds[0] = Some(FloatRect::new(58., 0., 6., 6.));
                }
                else if (i, j) == (0, size_x + 1) {
                    tile.sprite.set_texture(tex_man.get(TextureId::CornerTopRight), true);
                    tile.bounds[0] = Some(FloatRect::new(0., 58., 6., 6.));
                }
                else if (i, j) == (size_y + 1, size_x + 1) {
                    tile.sprite.set_texture(tex_man.get(TextureId::CornerBottomRight), true);
                    tile.bounds[0] = Some(FloatRect::new(0., 0., 6., 6.));
                }
                else if j == 0 {
                    tile.sprite.set_texture(tex_man.get(TextureId::WallLeft), true);
                    tile.bounds[0] = Some(FloatRect::new(58., 0., 6., 64.));
                }
                else if j == size_x + 1 {
                    tile.sprite.set_texture(tex_man.get(TextureId::WallRight), true);
                    tile.bounds[0] = Some(FloatRect::new(0., 0., 6., 64.));
                }
                else if i == 0 {
                    tile.sprite.set_texture(tex_man.get(TextureId::WallTop), true);
                    tile.bounds[0] = Some(FloatRect::new(0., 58., 64., 6.));
                }
                else if i == size_y + 1 {
                    tile.sprite.set_texture(tex_man.get(TextureId::WallBottom), true);
                    tile.bounds[0] = Some(FloatRect::new(0., 0., 64., 6.));
                }
                else {
                    tile.sprite.set_texture(tex_man.get(TextureId::Floor), true);
                    tile.is_solid = false;
                }
                tiles[i as usize].push(tile);
            }
        }

        Wagon {
            tiles: tiles,
            connected_to: [None, None]
        }
    }

    pub fn set_position2f(&mut self, dest_x: f32, dest_y: f32) {
        for i in 0..(self.tiles.len()) {
            for j in 0..(self.tiles[i].len()) {
                let current_x = self.tiles[i][j].sprite.get_position().x;
                let current_y = self.tiles[i][j].sprite.get_position().y;
                self.tiles[i][j].sprite.set_position2f(current_x + dest_x, current_y + dest_y);
            }
        }

        if let Some(ref mut x) = self.connected_to[0] {
            x.set_position2f(dest_x, dest_y);
        }
    }

    /// Connects wagon `other` to the *left* side of wagon `self`.
    pub fn connect(& mut self, other: &mut Wagon<'a>, tex_man: &'a TextureManager) {
        let self_height = self.tiles.len();
        let self_height_half = self_height / 2;
        let other_height = other.tiles.len();
        let other_width = other.tiles[0].len();
        let other_height_half = other_height / 2;

        self.tiles[self_height_half - 1][0].sprite.set_texture(tex_man.get(TextureId::ConnectorTop), true);
        self.tiles[self_height_half - 1][0].bounds[1] = Some(FloatRect::new(0., 58., 64., 6.));

        self.tiles[self_height_half][0].sprite.set_texture(tex_man.get(TextureId::Floor), true);
        self.tiles[self_height_half][0].is_solid = false;

        self.tiles[self_height_half + 1][0].sprite.set_texture(tex_man.get(TextureId::ConnectorBottom), true);
        self.tiles[self_height_half + 1][0].bounds[1] = Some(FloatRect::new(0., 0., 64., 6.));

        other.tiles[other_height_half - 1][other_width -1].sprite.set_texture(tex_man.get(TextureId::WallConnectedTop), true);
        other.tiles[other_height_half][other_width -1] = { let mut tile = Tile::new(); tile.is_solid = true; tile };
        other.tiles[other_height_half + 1][other_width -1].sprite.set_texture(tex_man.get(TextureId::WallConnectedBottom), true);
        other.set_position2f((self.tiles[0][0].sprite.get_position().x - ((other_width - 1) * TILE_SIZE_X as usize) as f32) as f32, self.tiles[0][0].sprite.get_position().y + ((self_height_half - other_height_half) * TILE_SIZE_Y as usize) as f32);
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
        }
        else if self.current_speed > 0. {
            self.current_speed -= self.accel * 2.;
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

        self.pfgrid_in.grid = vec![vec![PathfindingTile{ walkable:false }; max_height as usize]; total_width as usize];

        let mut prev_train_width = 0;
        for wagon in self.wagons.iter().rev() {
            let this_wagon_height = wagon.tiles.len();
            for (i, t) in wagon.tiles.iter().enumerate() {
                for (j, t) in t.iter().enumerate() {
                    self.pfgrid_in.grid[j + prev_train_width][i + (max_height - this_wagon_height)/2].walkable = !t.is_solid;
                }
            }
            prev_train_width += wagon.tiles[0].len() - 1;
        }

        self.pfgrid_out = self.pfgrid_in.clone();
        for (i, pft) in self.pfgrid_out.grid.iter_mut().enumerate() {
            for (j, pft) in pft.iter_mut().enumerate() {
                if !(i == 2 && j == 7) {
                    pft.walkable = !pft.walkable;
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct PathfindingTile {
    pub walkable: bool,
}

#[derive(Clone)]
pub struct PathfindingGrid {
    pub grid: Vec<Vec<PathfindingTile>>,
}

impl PathfindingGrid {
    pub fn new() -> Self {
        PathfindingGrid {
            grid: vec![],
        }
    }
}

pub trait Walkable {
    fn is_walkable(&self, x: i32, y: i32) -> bool;
}

impl Walkable for Vec<Vec<PathfindingTile>> {
    fn is_walkable(&self, x: i32, y: i32) -> bool
    {
        self[x as usize][y as usize].walkable
    }
}

struct TrainSearch<'a> {
    grid: &'a PathfindingGrid,
    start: (i32, i32),
    end: (i32, i32),
}

impl<'a> SearchProblem for TrainSearch<'a> {
    type Node = (i32, i32);
    type Cost = i32;
    type Iter = IntoIter<((i32, i32), i32)>;
    fn start(&self) -> (i32, i32) { self.start }
    fn is_end(&self, p: &(i32, i32)) -> bool { *p == self.end }
    fn heuristic(&self, &(p_x, p_y): &(i32, i32)) -> i32 {
        let (s_x, s_y) = self.end;
        (s_x - p_x).abs() + (s_y - p_y).abs()
    }
    fn neighbors(&mut self, &(x, y): &(i32, i32)) -> IntoIter<((i32, i32), i32)> {
        let mut vec = vec![];
        for i in -1 .. 1 + 1 {
            for k in -1 .. 1 + 1 {
                if !(i == 0 && k == 0)
                    // fucking corners
                    && !(i == -1 && k == -1)
                    && !(i == -1 && k == 1)
                    && !(i == 1 && k == -1)
                    && !(i == 1 && k == 1)
                    && x + i >= 0 && y + k >= 0
                    && x + i < self.grid.grid.len() as i32 && y + k < self.grid.grid[0].len() as i32
                    && self.grid.grid.is_walkable(x + i, y + k) {
                        vec.push(((x + i, y + k), 1));
                    }
            }
        }
        vec.into_iter()
    }
}

pub fn path(grid: &PathfindingGrid, start: (i32, i32), end: (i32, i32)) -> Option<VecDeque<(i32, i32)>> {
    let mut ts = TrainSearch{grid: grid, start: start, end: end };
    astar(&mut ts)
}
