use sdl2::{render::Canvas, video::Window};

use crate::{entities::Player, world::Map};

pub trait Display<'a> {
    fn display(&mut self,canvas: &mut Canvas<Window>, from: Option<Player>, map: Option<&Map>) -> Result<(),String>;
}