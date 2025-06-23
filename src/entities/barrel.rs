use std::{cell::RefCell, rc::Rc};

use sdl2::render::{Texture, TextureCreator};
use sdl2::image::LoadTexture;
use sdl2::video::WindowContext;

use crate::entities::Entity;
const T_PATH: &str = "assets/img/barrel.png";

pub struct Barrel<'a> {
    pos_x: f32,
    pos_y: f32,

    texture: Rc<RefCell<Texture<'a>>>
}

impl<'a> Barrel<'a> {
    pub fn new(x: f32, y: f32, tc: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let t = tc.load_texture(T_PATH)?;
        Ok(Self {
            pos_x: x,
            pos_y: y,
            texture: Rc::new(RefCell::new(t)),
        })
    }
}

impl<'a> Entity<'a> for Barrel<'a> {
    fn position(&self) -> (f32, f32) {
        (self.pos_x, self.pos_y)
    }

    fn direction(&self) -> Option<f32> {
        None
    }

    fn texture(&self) -> Rc<RefCell<Texture<'a>>> {
        self.texture.clone()
    }
}