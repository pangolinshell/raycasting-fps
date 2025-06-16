use sdl2::{render::Canvas, video::Window};

pub trait Display<'a> {
    fn display(&mut self,canvas: &mut Canvas<Window>) -> Result<(),String>;
}