use std::rc::Rc;

use sdl2::render::Texture;

use crate::entities::*;

/// static entities like pillars or barrels
pub struct NotMoving<'a> {
    position: (f32,f32),
    texture: Rc<Texture<'a>>,
}

impl<'a> NotMoving<'a> {
    pub fn new(x: f32,y: f32,texture: Texture<'a>) -> Self {
        Self { position: (x,y), texture: Rc::new(texture) }
    }
}

impl<'a> Entity<'a> for NotMoving<'a> {
    fn position(&self) -> (f32,f32) {
        self.position
    }

    fn direction(&self) -> f32 {
        0.0
    }

    fn entity_type(&self) -> entity::EntityType {
        entity::EntityType::UnLiving
    }

    fn texture(&self) -> Rc<sdl2::render::Texture<'a>> {
        self.texture.clone()
    }

    #[allow(unused_variables)]
    fn update(&mut self,ctx: Option<&mut Context<'a>>) -> Result<(),String> {
        Ok(())
    }
}