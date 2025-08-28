mod args;
use args::Args;
use clap::Parser;
use multiplayer_fps::{camera::Camera, display::Display, entities::Entity, frames::FramesCtrl, resources::TextureManager, world::{Map, Minimap}};

mod logic;
mod screen;
mod connection;
use connection::connection;


use std::{error::Error, net::SocketAddr, time::{Duration, Instant}};
use sdl2::{EventPump, event::Event, pixels::Color, rect::{FPoint, Rect}};
use sdl2::keyboard::Keycode;

use crate::{logic::{on_connection, shoot, update}, screen::window_init};

const WIN_TITLE: &str = "multiplayer fps";
const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 1024 + 256;

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

    let all_screen = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
    let render_zone = Rect::new(0, 0, 1280, 1024);
    let minimap_zone = Rect::new(0, 1025, 256, 256);
    let interface_zone = Rect::new(257, 1025, SCREEN_WIDTH - 256, 256);

    let args = Args::parse();
    let server: SocketAddr = format!("{}:{}",args.host,args.port).parse()?;
    let (tx,rx,udp_thread) = connection(server,args.nickname,Some(Duration::from_secs(40)))?;
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
        canvas.set_viewport(all_screen);
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_viewport(render_zone);
        frame_ctrl.start_frame();
        camera.inputs(&mut event_pump, frame_ctrl.dtime as f32,&map);
        if event_pump.keyboard_state().is_scancode_pressed(sdl2::keyboard::Scancode::Space) && shoot_cooldown.elapsed() >= Duration::from_secs(1) {
            shoot(&tx, camera, &nickname)?;
            shoot_cooldown = Instant::now();
        }
        match event(&mut event_pump) {
            1 => break,
            _ => (),
        }
        let mut rays = camera.cast_rays(map.clone(), SCREEN_WIDTH);
        rays.display(&mut canvas, Some(&texture_manager))?;
        let mut render_datas = vec![];
        for other in others.iter() {
            render_datas.push(other.into_render(camera, &map,&rays));
        }
        render_datas.sort();
        for mut rd in render_datas {
            rd.display(&mut canvas, Some(&texture_manager))?;
        }
        if camera.position != buff_cam_pos {
            buff_cam_pos = camera.position;
        }
        canvas.set_viewport(interface_zone);
        canvas.set_draw_color(Color::CYAN);
        canvas.fill_rect(Rect::new(0, 0, 1280, 1000))?;

        canvas.set_viewport(minimap_zone);
        let mut minimap = Minimap::new(&map, &FPoint::new(camera.position.0, camera.position.1), Color::GRAY, Color::BLACK);
        minimap.set_others(others.into_coordinates());
        minimap.set_target_pinpoint(Some(Color::YELLOW));
        minimap.display::<()>(&mut canvas, None)?;

        canvas.set_viewport(all_screen);
        canvas.present();
        update(&tx, &rx,&mut camera, &nickname,&mut others)?;
        frame_ctrl.end_frame();
    }
    udp_thread.kill();
    Ok(())
}