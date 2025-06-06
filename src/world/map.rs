use sdl2::rect::{Point};

use super::tiles::Tile;

#[derive(Debug,Clone)]
pub struct Map {
    pub layout: Vec<Tile>,
}

impl Map {
    pub fn from_bytes(u8_layout: Vec<Vec<u8>>) -> Self {
        let mut layout: Vec<Tile> = Vec::new();
        for (y,line) in u8_layout.iter().enumerate() {
        for (x,tile)in line.iter().enumerate() {
            let t = Tile::new(Point::new(x as i32, y as i32), *tile);
            layout.push(t);
        }
        }
        Self { layout}
    }

    pub fn from_file(path: &str) -> Result<Self,Box<dyn std::error::Error>> {
        let level = super::loader::Level::from_file(path)?;
        Ok(Map::from_bytes(level.layout))
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