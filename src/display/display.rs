use sdl2::{render::Canvas, video::Window};

use crate::{entities::Movable, resources::{TextureManager}};


pub trait Display {
    fn display<'l,T>(&mut self,canvas: &mut Canvas<Window>, textures: Option<&TextureManager<'l,T>>) -> Result<(),String>;
}