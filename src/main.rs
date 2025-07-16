mod server;
pub mod world;
pub mod entities;
pub mod display;
pub mod data;
pub mod error;
pub mod rays;
pub mod frames;
pub mod utils;
pub mod camera;
pub mod client;

use server::Instance;
use client::run;

fn main() {
    let instance = Instance::new(5000, 60);
    let _ = instance.run();
    run("127.0.0.1:5000", 0, String::from("Guest")).unwrap();
}
