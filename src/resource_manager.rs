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
