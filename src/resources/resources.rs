use std::collections::HashMap;
use std::rc::Rc;
use sdl2::{
    image::LoadTexture,
    render::{Canvas, Texture, TextureCreator},
    ttf::{Font, Sdl2TtfContext},
    video::{Window, WindowContext},
    Sdl, VideoSubsystem,
};

pub struct ResourcesManager<'ttf,'tex> {
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub ttf_context: &'ttf Sdl2TtfContext,
    pub texture_creator: TextureCreator<WindowContext>,

    pub window: Window,
    pub canvas: Canvas<Window>,
    pub fonts: HashMap<String, Rc<Font<'ttf, 'ttf>>>,
    pub textures: HashMap<String, Rc<Texture<'tex>>>,
}

impl<'ttf,'tex> ResourcesManager<'ttf,'tex> {
    pub fn new(
        window_name: &str,
        size: (u32, u32),
        ttf_context: &'ttf Sdl2TtfContext,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window(window_name, size.0, size.1)
            .position_centered()
            .build()?;
        let canvas = window.clone().into_canvas().build()?;
        let texture_creator = canvas.texture_creator();

        Ok(Self {
            sdl_context,
            video_subsystem,
            ttf_context,
            texture_creator,
            window,
            canvas,
            fonts: HashMap::new(),
            textures: HashMap::new(),
        })
    }

    pub fn load_font(
        &mut self,
        key: &str,
        path: &str,
        size: u16,
    ) -> Result<Rc<Font<'ttf, 'ttf>>, Box<dyn std::error::Error>> {
        if let Some(font) = self.fonts.get(key) {
            return Err(format!("Font key '{}' is already used", key).into());
        }

        let font = self.ttf_context.load_font(path, size)?;
        let font_rc = Rc::new(font);
        self.fonts.insert(key.to_string(), font_rc.clone());
        Ok(font_rc)
    }

    pub fn load_texture(
        &'tex mut self,
        key: &str,
        path: &str,
    ) -> Result<Rc<Texture<'tex>>, Box<dyn std::error::Error>> {
        if self.textures.contains_key(key) {
            return Err(format!("Texture key '{}' is already used", key).into());
        }

        let texture = self.texture_creator.load_texture(path)?;
        let texture_rc = Rc::new(texture);
        self.textures.insert(key.to_string(), texture_rc.clone());
        Ok(texture_rc)
    }
}
