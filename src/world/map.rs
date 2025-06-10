use sdl2::{image::LoadTexture, rect::Point, render::TextureCreator, video::WindowContext};
use crate::display::{self, TextureMap};
use std::rc::Rc;

use super::tiles::Tile;

#[derive(Clone)]
pub struct Map<'a> {
    pub layout: Vec<Tile>,
    pub textures: display::TextureMap<'a>
}

impl<'a> Map<'a> {
    pub fn from_bytes(u8_layout: Vec<Vec<u8>>) -> Self {
        let mut layout: Vec<Tile> = Vec::new();
        for (y,line) in u8_layout.iter().enumerate() {
        for (x,tile)in line.iter().enumerate() {
            let t = Tile::new(Point::new(x as i32, y as i32), *tile);
            layout.push(t);
        }
        }
        Self { layout, textures: TextureMap::new()}
    }

    pub fn from_file(path: &str,texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self,Box<dyn std::error::Error>> {
        let level = super::loader::Level::from_file(path)?;
        let dir = level.textures.directory;
        let mut map = Map::from_bytes(level.layout);
        for (code,filename) in level.textures.tiles {
            let texture = texture_creator.load_texture(format!("{}{}",dir,filename))?;
            map.textures.add_texture(code, Rc::new(texture))?;
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