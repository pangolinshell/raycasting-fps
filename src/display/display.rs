use sdl2::{render::Canvas, video::Window};

pub trait Display {
    fn display(&mut self,canvas: &mut Canvas<Window>,textures: Option<super::TextureMap>) -> Result<(),String>;
}