#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(inclusive_range_syntax)]

extern crate sfml;
extern crate rand;
extern crate astar;

mod state_stack;
mod resource_manager;
use resource_manager::Resources;
mod particle_manager;
mod actor;
mod menu;
mod wagon;
mod game;
use game::Game;
mod world;
mod camera;

mod game_consts {
    pub const TILE_SIZE_X: u32 = 64;
    pub const TILE_SIZE_Y: u32 = 64;

    pub const WINDOW_SIZE_X: u32 = 1600;
    pub const WINDOW_SIZE_Y: u32 = 900;
}

fn main() {
    let resources = Resources::new();
    let mut game = Game::new(&resources);

    game.run().unwrap();
}
