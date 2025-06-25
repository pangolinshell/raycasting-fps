use std::collections::HashMap;
use std::rc::Rc;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct ResourceManager<'a> {
    creator: &'a TextureCreator<WindowContext>,
    textures: HashMap<String, Rc<Texture<'a>>>,
}

impl<'a> ResourceManager<'a> {
    pub fn new(creator: &'a TextureCreator<WindowContext>) -> Self {
        Self {
            creator,
            textures: HashMap::new(),
        }
    }

    pub fn load(&mut self, id: &str, path: &str) -> Result<(), String> {
        let texture = self.creator.load_texture(path)?;
        self.textures.insert(id.to_string(), Rc::new(texture));
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<Rc<Texture<'a>>> {
        self.textures.get(id).cloned()
    }
}
