use std::{cell::RefCell, collections::HashMap, rc::Rc};

use sdl2::{render::{Canvas, Texture}, video::Window};

pub trait Display<'a> {
    fn display(&mut self,canvas: &mut Canvas<Window>,textures: HashMap<u8,Rc<RefCell<Texture<'a>>>>,missing: Option<Rc<RefCell<Texture<'a>>>> ) -> Result<(),String>;
}