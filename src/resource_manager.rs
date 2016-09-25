use std::collections::HashMap;
use std::hash::Hash;
extern crate sfml;
use sfml::graphics::{Texture, Font};
use sfml::audio::{Music, SoundBuffer};
use wagon::Direction;

pub trait Resource: Sized {
    fn new_from_file(filename: &str) -> Option<Self>;
}

impl Resource for Texture {
    fn new_from_file(filename: &str) -> Option<Self> {
        Texture::new_from_file(filename)
    }
}

impl Resource for Font {
    fn new_from_file(filename: &str) -> Option<Self> {
        Font::new_from_file(filename)
    }
}

impl Resource for Music {
    fn new_from_file(filename: &str) -> Option<Self> {
        Music::new_from_file(filename)
    }
}

impl Resource for SoundBuffer {
    fn new_from_file(filename: &str) -> Option<Self> {
        SoundBuffer::new(filename)
    }
}

pub struct ResourceManager<I, R> {
    pub resource_map: HashMap<I, R>,
}

impl<I: Eq + Hash, R: Resource> ResourceManager<I, R> {
    pub fn new() -> Self {
        ResourceManager { resource_map: HashMap::<I, R>::new() }
    }

    pub fn load(&mut self, identifier: I, filename: &str) {
        let resource = R::new_from_file(filename).unwrap();
        self.resource_map.insert(identifier, resource);
    }

    pub fn get(&self, identifier: I) -> &R {
        if let Some(resource) = self.resource_map.get(&identifier) {
            resource
        } else {
            panic!("Tried to access nonexistant index in resource map")
        }
    }

    pub fn get_mut(&mut self, identifier: I) -> &mut R {
        if let Some(resource) = self.resource_map.get_mut(&identifier) {
            resource
        } else {
            panic!("Tried to access nonexistant index in resource map")
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum TextureId {
    Floor,
    WallTop,
    WallBottom,
    WallLeft,
    WallRight,
    CornerTopLeft,
    CornerTopRight,
    CornerBottomLeft,
    CornerBottomRight,
    ConnectorTop,
    ConnectorBottom,
    WallConnectedTop,
    WallConnectedBottom,
    Background,
    Actor,
    Enemy,
    DoorOpen(Direction),
    DoorClosed(Direction),
}
#[derive(PartialEq, Eq, Hash)]
pub enum FontId {
    Arial,
    Joystix,
}

#[derive(PartialEq, Eq, Hash)]
pub enum MusicId {
    Train,
    Screech,
}

#[derive(PartialEq, Eq, Hash)]
pub enum SoundId {

}

pub type TextureManager = ResourceManager<TextureId, Texture>;
pub type FontManager = ResourceManager<FontId, Font>;
pub type MusicManager = ResourceManager<MusicId, Music>;
pub type SoundManager = ResourceManager<SoundId, SoundBuffer>;

pub struct Resources {
    pub fm: FontManager,
    pub tm: TextureManager,
    pub sm: SoundManager,
}

impl Resources {
    pub fn new() -> Self {
        let mut fm = FontManager::new();
        fm.load(FontId::Arial, "res/arial.ttf");
        fm.load(FontId::Joystix, "res/joystix-mono.ttf");

        let mut tm = TextureManager::new();
        tm.load(TextureId::Floor, "res/floor.png");
        tm.load(TextureId::WallTop, "res/wall_top.png");
        tm.load(TextureId::WallBottom, "res/wall_bottom.png");
        tm.load(TextureId::WallLeft, "res/wall_left.png");
        tm.load(TextureId::WallRight, "res/wall_right.png");
        tm.load(TextureId::CornerTopLeft, "res/corner_topleft.png");
        tm.load(TextureId::CornerTopRight, "res/corner_topright.png");
        tm.load(TextureId::CornerBottomLeft, "res/corner_bottomleft.png");
        tm.load(TextureId::CornerBottomRight, "res/corner_bottomright.png");
        tm.load(TextureId::ConnectorTop, "res/connector_top.png");
        tm.load(TextureId::ConnectorBottom, "res/connector_bottom.png");
        tm.load(TextureId::WallConnectedTop, "res/wall_connected_top.png");
        tm.load(TextureId::WallConnectedBottom,
                "res/wall_connected_bottom.png");
        tm.load(TextureId::Background, "res/bg.png");
        tm.load(TextureId::Actor, "res/actor.png");
        tm.load(TextureId::Enemy, "res/enemy.png");
        tm.load(TextureId::DoorOpen(Direction::North), "res/door_north.png");
        tm.load(TextureId::DoorOpen(Direction::South), "res/door_south.png");
        tm.load(TextureId::DoorOpen(Direction::West), "res/door_west.png");
        tm.load(TextureId::DoorOpen(Direction::East), "res/door_east.png");
        tm.load(TextureId::DoorClosed(Direction::North), "res/door_closed_north.png");
        tm.load(TextureId::DoorClosed(Direction::South), "res/door_closed_south.png");
        tm.load(TextureId::DoorClosed(Direction::West), "res/door_closed_west.png");
        tm.load(TextureId::DoorClosed(Direction::East), "res/door_closed_east.png");

        let mut sm = SoundManager::new();
//        sm.

        Resources {
            fm: fm,
            tm: tm,
            sm: sm,
        }
    }
}
