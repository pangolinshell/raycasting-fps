// mod server;
// pub mod resources;

// pub mod world;
// pub mod entities;
// pub mod display;
// pub mod data;
// pub mod error;
// pub mod rays;
// pub mod frames;
// pub mod utils;
// pub mod camera;
// pub mod client;

use std::error::Error;

// use server::Instance;
// use client::run;
use multiplayer_fps::resources::ResourcesManager;
use multiplayer_fps::world::n_loader::Loader;

fn main() -> Result<(),Box<dyn Error>> {
    // let instance = Instance::new(5000, 60);
    // let _ = instance.run();
    // run("127.0.0.1:5000", 0, String::from("Guest")).unwrap();
    let loader = Loader::from_file("conf/map1.jsonc")?;
    let mut rm = ResourcesManager::new("test", (120,120))?;
    rm.load_font_from_resources(loader.get_resources().clone())?;
    Ok(())
}
