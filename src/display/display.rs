use sdl2::{render::Canvas, video::Window};

use crate::{camera::Camera, world::Map};

pub trait Display<'a> {
    fn display(&mut self,canvas: &mut Canvas<Window>, from: Option<Camera>, map: Option<&Map>) -> Result<(),String>;
}