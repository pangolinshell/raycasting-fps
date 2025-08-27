use std::collections::HashMap;

use super::tiles::Tile;

#[derive(Clone)]
pub struct Map {
    pub layout: Vec<Tile>,
    pub texture_map: HashMap<u8,String>,
    pub missing_texture: String,
    pub spawn_points: Vec<Tile>
}

impl Map {
    pub fn new(layout: Vec<Tile>,t_map: HashMap<u8,String>,missing: String,spawn_points: Vec<Tile>) -> Self {
        Self { layout, texture_map: t_map,missing_texture: missing,spawn_points}
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