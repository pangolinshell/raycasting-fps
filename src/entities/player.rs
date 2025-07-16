use std::rc::Rc;

use sdl2::render::Texture;

use crate::{data::{Host, Update}, entities::Entity};

pub struct Player<'a> {
    pub data: Host,

    pub texture: Rc<Texture<'a>>
}

impl<'a> Player<'a> {
    pub fn new(host: Host,texture: Rc<Texture<'a>>) -> Self {
        Self { data: host, texture: texture.clone() }
    }

    pub fn update(&mut self,data: Update) -> u8 {
        self.data.update(data)
    }
}

impl<'a> Entity<'a> for Player<'a> {
    fn direction(&self) -> f32 {
        self.data.d
    }

    fn position(&self) -> (f32,f32) {
        (self.data.x,self.data.y)
    }

    fn update(&mut self,ctx: Option<&mut super::Context<'a>>) -> Result<(),String> {
        Ok(())
    }

    fn entity_type(&self) -> super::entity::EntityType {
        super::entity::EntityType::Player
    }

    fn texture(&self) -> std::rc::Rc<sdl2::render::Texture<'a>> {
        self.texture.clone()
    }
}