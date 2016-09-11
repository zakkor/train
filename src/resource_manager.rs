use std::collections::HashMap;
use std::hash::Hash;
extern crate sfml;
use sfml::graphics::{Texture, Font};

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

pub struct ResourceManager<I, R> {
    resource_map: HashMap<I, Box<R>>
}

impl<I: Eq + Hash, R: Resource> ResourceManager<I, R> {
    pub fn new() -> Self {
        ResourceManager {
            resource_map: HashMap::<I, Box<R>>::new()
        }
    }

    pub fn load(&mut self, identifier: I, filename: & str) {
        let resource = R::new_from_file(filename).unwrap();
        self.resource_map.insert(identifier, Box::new(resource));
    }

    pub fn get(&self, identifier: I) -> &Box<R> {
        match self.resource_map.get(&identifier) {
            Some(resource) => resource,
            None => panic!("Tried to access nonexistant index in resource map")
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
}
#[derive(PartialEq, Eq, Hash)]
pub enum FontId {
    Arial,
    Joystix
}

pub type TextureManager = ResourceManager<TextureId, Texture>;
pub type FontManager = ResourceManager<FontId, Font>;


pub struct Resources {
    pub fm: FontManager,
    pub tm: TextureManager,
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
        tm.load(TextureId::WallConnectedBottom, "res/wall_connected_bottom.png");
        tm.load(TextureId::Background, "res/bg.png");

        Resources {
            fm: fm,
            tm: tm,
        }
    }
}
