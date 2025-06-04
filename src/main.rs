extern crate sdl2;
mod utils;
mod frames;

use multiplayer_fps_v3::display::Display;
use multiplayer_fps_v3::world::Map;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::ttf::{self, Font};

use multiplayer_fps_v3::entities::{Entity, Player};

const WIN_RES: (u32,u32) = (1280, 1024);


fn clear(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
}

fn display_fps(c: Point,font: &Font,fps: f64, canvas: &mut Canvas<Window>) -> Result<(),String> {
    let tc = canvas.texture_creator();
    let s = font.render(format!("fps: {:.0}",fps).as_str()).blended(Color::WHITE).unwrap();
    let t= tc.create_texture_from_surface(s).unwrap();
    canvas.copy(&t, None, Rect::new(c.x,c.y, 100, 20))
}

fn event(e:&mut EventPump) -> Option<u32>{
    for event in e.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return Some(0);
            },
            _ => {}
        }
    }
    return None;
}

fn map() -> Map {
    let world_map: Vec<Vec<u8>> = vec![
    vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    vec![1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
    vec![1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
    vec![1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
    vec![1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
    vec![1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
    vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    vec![1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    vec![1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    vec![1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    vec![1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    vec![1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    vec![1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    vec![1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
];

    Map::from_bytes(world_map)
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let ttf = ttf::init().unwrap();
    let font = ttf.load_font("./assets/fonts/ProggyCleanNerdFont-Regular.ttf", 16).unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", WIN_RES.0, WIN_RES.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let map = map();
    let mut player = Player::new(22.0, 12.0, utils::angles::degrees_to_rad(180.0));
    let mut loop_ctrl = frames::FramesCtrl::init(60);


    loop {
        clear(&mut canvas);
        loop_ctrl.start_frame();
        if event(&mut event_pump) == Some(0) {
            break;
        }

        // -- start game loop --
        player.inputs(&mut event_pump, loop_ctrl.dtime as f32);
        let mut r = player.cast_rays(map.clone(), WIN_RES.0);
        r.display(&mut canvas).unwrap();


        // -- end game loop --
        display_fps(Point::new(0, 0), &font, loop_ctrl.fps(), &mut canvas).unwrap();
        canvas.present();
        loop_ctrl.end_frame();
    }
}