use std::rc::Rc;

use sdl2::render::Texture;

use crate::entities::Entity;

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

    fn texture(&self) -> Rc<sdl2::render::Texture<'a>> {
        self.texture.clone()
    }

    fn update(&self) -> Result<(),String> {
        Err(format!("barrel has no update behavior (it's a barrel)"))
    }
}