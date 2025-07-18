use sdl2::{render::Canvas, video::Window};

use crate::{entities::Movable, resources::{TextureManager}};


pub trait Display {
    fn display<'l,M,T>(&mut self,canvas: &mut Canvas<Window>, from: Option<M>, textures: Option<TextureManager<'l,T>>) -> Result<(),String> where M: Movable;
}