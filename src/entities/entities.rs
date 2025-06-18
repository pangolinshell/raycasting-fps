use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use sdl2::{rect::{FPoint, Point}, render::Texture};

use crate::{display::Display, entities::Player};

#[derive(Clone)]
pub struct Entity<'a> {
    pub id: u32,
    // position of the entities
    pub x: f32,  
    pub y: f32,

    // Direction where the entity is looking
    pub look: Option<f32>,

    // texture of the entity
    pub texture: Rc<RefCell<Texture<'a>>>,

    pub pov: Player
}

pub struct Entites<'a> {
    all: Vec<Entity<'a>>,
    pub player: Player,
}

impl<'a> PartialEq for Entity<'a> {
    fn eq(&self, other: &Self) -> bool {
        let delta_a = delta((self.x,self.y), self.pov.position);
        let delta_b = delta((other.x,other.y), other.pov.position);
        delta_a == delta_b
    }
}

impl<'a> PartialOrd for Entity<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let delta_a = delta((self.x,self.y), self.pov.position);
        let delta_b = delta((other.x,other.y), other.pov.position);
        if delta_a < delta_b {
            Some(Ordering::Less)
        } else if delta_a > delta_b {
            Some(Ordering::Greater)
        } else if delta_a == delta_b {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

impl<'a> Eq for Entity<'a> {}

impl<'a> Ord for Entity<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let delta_a = delta((self.x,self.y), self.pov.position);
        let delta_b = delta((other.x,other.y), other.pov.position);
        if delta_a < delta_b {
            Ordering::Less
        } else if delta_a > delta_b {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
            
    }
}

fn delta(a: (f32, f32), b: (f32, f32)) -> f32 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    (dx * dx + dy * dy).sqrt()
}



impl<'a> Display<'a> for Entites<'a> {
    fn display(&mut self,canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(),String> {
        self.all.sort();
        
        todo!()
    }
}

impl<'a> Entity<'a> {
    pub fn new(id: u32,pos: FPoint,texture:Rc<RefCell<Texture<'a>>>,player: Player) -> Self {
        Self { id, x: pos.x, y: pos.y, look: None, texture: texture.clone(), pov: player }
    }
}