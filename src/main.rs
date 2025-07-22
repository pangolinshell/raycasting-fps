use multiplayer_fps::resources::{ResourceManager,TextureManager};

use sdl2::video::WindowContext;


fn main() -> Result<(), String> {
    let sdl_ctx = sdl2::init()?;
    let video_subsys = sdl_ctx.video()?;
    let win = video_subsys.window("test", 100, 100).position_centered().build().map_err(|e| e.to_string())?;
    let mut canvas = win
            .into_canvas()
            .software()
            .build()
            .map_err(|e| e.to_string())?;
    let t_loader = canvas.texture_creator();
    // Specify the key type (e.g., String) and value type (e.g., Texture)
    let mut texture_manager: TextureManager<WindowContext> = ResourceManager::new(&t_loader);
    texture_manager.load("redbrick","assets/img/redbrick.png")?;
    match texture_manager.get("redbrick") {
        Some(v) => println!("found"),
        None => println!("not found"),
    }
    Ok(())
}