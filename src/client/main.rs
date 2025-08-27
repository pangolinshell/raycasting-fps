mod args;
use args::Args;
use clap::Parser;
use multiplayer_fps::{camera::Camera, display::Display, entities::Entity, frames::FramesCtrl, resources::TextureManager, world::Map};

mod logic;
mod screen;
mod connection;
use connection::connection;


use std::{error::Error, net::SocketAddr, time::{Duration, Instant}};
use sdl2::{event::Event, pixels::Color, EventPump};
use sdl2::keyboard::Keycode;

use crate::{logic::{on_connection, shoot, update}, screen::window_init};

const WIN_TITLE: &str = "multiplayer fps";
const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 1024;

const TARGET_FPS: u32 = 60;


fn event(e:&mut EventPump) -> u32{
    for event in e.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return 1;
            },
            _ => {}
        }
    }
    return 0;
}


fn main() -> Result<(),Box<dyn Error>> {
    let args = Args::parse();
    let server: SocketAddr = format!("{}:{}",args.host,args.port).parse()?;
    let (tx,rx) = connection(server,args.nickname,Some(Duration::from_secs(40)))?;
    let (player,mut others,map_loader) = on_connection(&rx)?;
    let nickname = player.nickname;

    let sdl = sdl2::init()?;
    let mut event_pump = sdl.event_pump()?;
    let window = window_init(WIN_TITLE, SCREEN_WIDTH, SCREEN_HEIGHT, sdl)?;
    let mut canvas = window.into_canvas().accelerated().build()?;
    let texture_creator = canvas.texture_creator();

    let mut texture_manager = TextureManager::new(&texture_creator);
    let textures = map_loader.get_resources().textures()?;
    let textures_ref: std::collections::HashMap<&str, &str> = textures.iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    texture_manager.load_from_map(textures_ref)?;
    let map = Map::from(&map_loader);
    let mut camera = Camera::new(player.x, player.y, player.d);
    let mut buff_cam_pos: (f32,f32) = camera.position;
    let mut frame_ctrl = FramesCtrl::init(TARGET_FPS);
    let mut shoot_cooldown = Instant::now();
    loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        frame_ctrl.start_frame();
        camera.inputs(&mut event_pump, frame_ctrl.dtime as f32);
        if event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::Space) && shoot_cooldown.elapsed() >= Duration::from_secs(1) {
            shoot(&tx, camera, &nickname)?;
            shoot_cooldown = Instant::now();
        }
        match event(&mut event_pump) {
            1 => break,
            _ => (),
        }
        let mut rays = camera.cast_rays(map.clone(), SCREEN_WIDTH);
        rays.display(&mut canvas, None::<multiplayer_fps::entities::Player>, Some(&texture_manager))?;
        let mut render_datas = vec![];
        for other in others.iter() {
            render_datas.push(other.into_render(camera, &map));
        }
        render_datas.sort();
        for mut rd in render_datas {
            rd.display(&mut canvas, &texture_manager)?;
        }
        if camera.position != buff_cam_pos {
            println!("x :{},y: {}",camera.position.0,camera.position.1);
            buff_cam_pos = camera.position;
        }
        canvas.present();
        update(&tx, &rx,&mut camera, &nickname,&mut others)?;
        frame_ctrl.end_frame();
    }
    Ok(())
}