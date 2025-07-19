use sdl2::{render::Canvas, video::Window, EventPump, Sdl, VideoSubsystem};
use std::rc::Rc;

type Error = Box<dyn std::error::Error>;

/// init and setup the window
pub fn window_init(title: &str,width: u32,height: u32,sdl: Sdl) -> Result<Window,Error> {
    let video_subsys = sdl.video()?;
    let window = video_subsys.window(title, width, height)
        .opengl()
        .position_centered()
        .build()?;
    Ok(window)
}