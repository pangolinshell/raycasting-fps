use std::{cell::{RefCell}, rc::Rc};

use sdl2::render::Texture;

pub struct Entity {
    pub id: u32,
    // position of the entities
    pub x: f32,
    pub y: f32,

    // Direction where the entity is looking
    pub look: Option<f32>,

    // texture of the entity
    pub texture: Rc<RefCell<Texture<'static>>>,
}