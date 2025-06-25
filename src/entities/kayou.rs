use std::rc::Rc;
use sdl2::render::Texture;

use crate::entities::Entity;
const T_PATH: &str = "assets/img/kayou.png";

pub struct Kayou<'a> {
    pos_x: f32,
    pos_y: f32,

    texture: Rc<Texture<'a>>
}

impl<'a> Kayou<'a> {
    pub fn new(x: f32, y: f32, t: Rc<Texture<'a>>) -> Result<Self, String> {
        Ok(Self {
            pos_x: x,
            pos_y: y,
            texture: t,
        })
    }
}

impl<'a> Entity<'a> for Kayou<'a> {
    fn position(&self) -> (f32, f32) {
        (self.pos_x, self.pos_y)
    }

    fn direction(&self) -> Option<f32> {
        None
    }

    fn texture(&self) -> Rc<Texture<'a>> {
        self.texture.clone()
    }
}