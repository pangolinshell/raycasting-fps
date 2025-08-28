use sdl2::rect::Point;

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum TileType {
    Void,
    Wall(u8),
}

#[derive(Debug,Copy,Clone)]
pub struct Tile {
    pub t_type: TileType,
    pub pos: Point,
}

impl Tile {
    pub fn new(pos: Point, type_value: u8) -> Tile {
        let ttype = match type_value {
            0 => TileType::Void,
            _ => TileType::Wall(type_value),
        };

        Self { t_type: ttype, pos: pos }
    }

    pub fn is_wall(&self) -> bool {
        match self.t_type {
            TileType::Wall(_) => true,
            _ => false,
        }
    }

    pub fn get_tvalue(&self) -> Option<u8> {
        match self.t_type {
            TileType::Void => return None,
            TileType::Wall(w) => Some(w),
        }
    }
}