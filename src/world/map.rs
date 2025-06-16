use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, TextureAccess};
use sdl2::{image::LoadTexture, rect::Point, render::TextureCreator, video::WindowContext};
use crate::error;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use super::tiles::Tile;

#[derive(Clone)]
pub struct Map<'a> {
    pub layout: Vec<Tile>,
    pub textures: HashMap<u8, Rc<RefCell<Texture<'a>>>>,
    pub missing: Rc<RefCell<Texture<'a>>>,
}

impl<'a> Map<'a> {
    pub fn from_bytes(u8_layout: Vec<Vec<u8>>, missing: Rc<RefCell<Texture<'a>>>) -> Self {
        let mut layout: Vec<Tile> = Vec::new();
        for (y,line) in u8_layout.iter().enumerate() {
        for (x,tile)in line.iter().enumerate() {
            let t = Tile::new(Point::new(x as i32, y as i32), *tile);
            layout.push(t);
        }
        }
        Self { layout, textures: HashMap::new(), missing}
    }

    pub fn from_file(path: &str, texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, Box<dyn std::error::Error>> {
        let level = super::loader::Level::from_file(path)?;
        let dir = level.textures.directory;
        let missing = texture_creator
        .create_texture(
            PixelFormatEnum::RGBA8888,
            TextureAccess::Streaming,
            64,
            64,
        )?;
        let mut map = Map::from_bytes(level.layout,Rc::new(RefCell::new(missing)));
        for (code, filename) in level.textures.tiles {
            let texture = texture_creator.load_texture(format!("{}{}", dir, filename))?;
            match map.textures.insert(code, Rc::new(RefCell::new(texture)).clone()) {
                Some(_) => return Err(Box::new(error::Error::new(format!("code {} has been used at least twice please correct", code).as_str()))),
                None => {},
            };
        };
        Ok(map)
    }

    pub fn get_tile(&self,x: i32,y:i32) -> Option<Tile> {
        for t in self.layout.clone() {
            if (t.pos.x,t.pos.y) == (x,y) {
                return Some(t);
            }
        }
        None
    }

    pub fn is_wall(&self,x:i32,y:i32) -> Option<bool> {
        let t = match self.get_tile(x, y) {
            Some(t) => t,
            None => return None,
        };
        Some(t.is_wall())
    }

    pub fn get_tvalue(&self,x:i32,y:i32) -> Option<u8> {
        let t = self.get_tile(x, y)?;
        t.get_tvalue()
    }
 }