use std::error::Error;
use std::{collections::HashMap};
use std::rc::Rc;
use sdl2::{
    image::LoadTexture,
    render::{Canvas, Texture, TextureCreator},
    // ttf::{Font, Sdl2TtfContext},
    video::{Window, WindowContext},
    Sdl, VideoSubsystem,
};

use crate::world::n_loader::{Resources};

pub struct ResourcesManager<'a> {
    // pub sdl_context: Sdl,
    // pub video_subsystem: VideoSubsystem,
    pub texture_creator: TextureCreator<WindowContext>,
    // pub window: Window,
    // pub canvas: Canvas<Window>,
    pub textures: HashMap<String, Rc<Texture<'a>>>,

    // pub ttf_context: &'a Sdl2TtfContext,
    // pub fonts: HashMap<String, Rc<Font<'a, 'a>>>,
}

impl<'a> ResourcesManager<'a> {
    pub fn new(
        // window_name: &str,
        // // ttf_context: &'a Sdl2TtfContext,
        // size: (u32, u32),
        canvas: &Canvas<Window>
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // let sdl_context = sdl2::init()?;
        // // let ttf_context: Sdl2TtfContext = sdl2::ttf::init()?;
        // let video_subsystem = sdl_context.video()?;
        // let window = video_subsystem
        //     .window(window_name, size.0, size.1)
        //     .position_centered()
        //     .build()?;
        // let canvas = window.clone().into_canvas().build()?;
        let texture_creator = canvas.texture_creator();

        Ok(Self {
            // sdl_context,
            // video_subsystem,
            // ttf_context: ttf_context,
            texture_creator,
            // window, 
            // canvas,
            // fonts: HashMap::new(),
            textures: HashMap::new(),
        })
    }

    // pub fn load_font(
    //     &'a mut self,
    //     key: &str,
    //     path: &str,
    //     size: u16,
    // ) -> Result<Rc<Font<'a, 'a>>, Box<dyn std::error::Error>> {
    //     if self.fonts.contains_key(key) {
    //         return Err(format!("Font key '{}' is already used", key).into());
    //     }

    //     let font = self.ttf_context.load_font(path, size)?;
    //     let font_rc = Rc::new(font);
    //     self.fonts.insert(key.to_string(), font_rc.clone());
    //     Ok(font_rc)
    // }

    // pub fn load_fonts(
    //     &'a mut self,
    //     fonts: &HashMap<String, (String, u16)>,
    // ) -> Result<(), Box<dyn std::error::Error>> {
    //     for (key, (path, size)) in fonts {
    //         if self.fonts.contains_key(key) {
    //             return Err(format!("Font key '{}' is already used", key).into());
    //         }

    //         let font = self.ttf_context.load_font(path, *size)?;
    //         let font_rc = Rc::new(font);
    //         self.fonts.insert(key.clone(), font_rc);
    //     }
    //     Ok(())
    // }

    pub fn load_texture(
        &'a mut self,
        key: &str,
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.textures.contains_key(key) {
            return Err(format!("Texture key '{}' is already used", key).into());
        }

        let texture = self.texture_creator.load_texture(path)?;
        let texture_rc = Rc::new(texture);
        self.textures.insert(key.to_string(), texture_rc);
        Ok(())
    }

    pub fn load_textures(
        &'a mut self,
        textures: &HashMap<String, String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (key, path) in textures {
            if self.textures.contains_key(key) {
                return Err(format!("Texture key '{}' is already used", key).into());
            }

            let texture = self.texture_creator.load_texture(path)?;
            let texture_rc = Rc::new(texture);
            self.textures.insert(key.clone(), texture_rc);
        }
        Ok(())
    }

    pub fn load_textures_from_resources(
        &'a mut self,
        r: &Resources,
    ) -> Result<(), Box<dyn Error>> {
        let t = r.textures()?;
        self.load_textures(&t)?;
        Ok(())
    }

    // pub fn load_font_from_resources(
    //     &'a mut self,
    //     r: Resources,
    // ) -> Result<(), Box<dyn Error>> {
    //     let t = r.fonts()?;
    //     self.load_fonts(&t)?;
    //     Ok(())
    // }
}

