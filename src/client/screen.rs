use std::{collections::HashMap, error::Error};

use sdl2::{render::Canvas, ttf::{self, Font, Sdl2TtfContext}, video::Window};

const WIN_RES: (u32,u32) = (1280, 1024);

pub struct Screen<'a> {
    window: Window,
    ttf_context: Sdl2TtfContext,

    fonts: HashMap<&'a str, Font<'a, 'a>>,
    canvas: HashMap<&'a str,Canvas<Window>>
}

pub fn init<'a>() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = ttf::init()?;

    let window = video_subsystem.window("rust-sdl2 demo", WIN_RES.0, WIN_RES.1)
        .position_centered()
        .build()?;

    Ok(())
}