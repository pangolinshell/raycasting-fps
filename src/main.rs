extern crate sdl2;
mod utils;
mod frames;
use std::cell::RefCell;
use std::rc::Rc;

use multiplayer_fps_v3::display::{Display, Minimap};
use multiplayer_fps_v3::world::{ Map};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{FPoint, Point, Rect};
use sdl2::render::{Canvas};
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::ttf::{self, Font};

use multiplayer_fps_v3::entities::{
    // Entites, 
    self, Barrel, Entity, Player};

const WIN_RES: (u32,u32) = (1280, 1024);


fn clear(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
}

fn display_fps(c: Point,font: &Font,fps: f64, canvas: &mut Canvas<Window>, player: Player) -> Result<(),String> {
    let tc = canvas.texture_creator();
    let s = font.render(format!("fps: {:.0} pos: {:.2},{:.2}",fps,player.position.0,player.position.1).as_str()).blended(Color::WHITE).unwrap();
    let t= tc.create_texture_from_surface(s).unwrap();
    canvas.copy(&t, None, Rect::new(c.x,c.y, 200, 20))
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
    let texture_creator = canvas.texture_creator();

    let map = Map::from_file("conf/map2.jsonc",&texture_creator).unwrap();
    let player = Rc::new(RefCell::new(Player::new(22.0, 12.0, utils::angles::degrees_to_rad(180.0))));
    entities::init_player(player.clone());
    let mut loop_ctrl = frames::FramesCtrl::init(60);
    let mut minimap = Minimap::new(map.clone(), 800/24, 800, Point::new(0, 0));

    let minimap_win = video_subsystem.window("minimap", 800, 800).position_centered().build().unwrap();
    let mut minimap_canvas = minimap_win.into_canvas().build().unwrap();

    // let mut entites = Entites::init( map.clone(),player.clone());

    let mut placeholder = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA8888, 64, 64)
        .expect("Erreur lors de la création de la texture");

    placeholder
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..64 {
                for x in 0..64 {
                    let offset = y * pitch + x * 4;
                    buffer[offset] = 0;       // Rouge
                    buffer[offset + 1] = 255; // Vert
                    buffer[offset + 2] = 0;   // Bleu
                    buffer[offset + 3] = 255; // Alpha
                }
            }
        })
        .expect("Erreur lors du lock de la texture");
    // let barrel = Entity::new(0, FPoint::new(13.5, 16.5), Rc::new(RefCell::new(placeholder)), player.clone());

    let mut barrel = Barrel::new(13.5, 16.5, &texture_creator).unwrap();
    
    // entites.add(barrel.clone());

    loop {
        clear(&mut canvas);
        minimap_canvas.set_draw_color(Color::BLACK);
        minimap_canvas.clear();
        loop_ctrl.start_frame();
        if event(&mut event_pump) == Some(0) {
            break;
        }

        // -- start game loop --
        {
            let ppos = player.borrow().position;
            let rect = Rect::from_center(Point::new((ppos.0 * 800.0/24.0) as i32,(ppos.1 * 800.0/24.0) as i32), 800/26,800/26);
            minimap_canvas.set_draw_color(Color::YELLOW);
            minimap_canvas.fill_rect(rect).unwrap();
            minimap_canvas.set_draw_color(Color::GREEN);
            // minimap_canvas.fill_rect(Rect::from_center(Point::new((barrel.x * 800.0/24.0) as i32, (barrel.y * 800.0/24.0) as i32), 30, 30)).unwrap();
        }
        
        minimap.display(&mut minimap_canvas).unwrap();
        player.borrow_mut().inputs(&mut event_pump, loop_ctrl.dtime as f32);
        let mut r = player.borrow_mut().cast_rays(map.clone(), WIN_RES.0);
        r.display(&mut canvas).unwrap();

        barrel.display(&map,&mut canvas).unwrap();

        // entites.render(&mut canvas).unwrap();

        // -- end game loop --
        display_fps(Point::new(0, 0), &font, loop_ctrl.fps(), &mut canvas,*player.borrow()).unwrap();

        minimap_canvas.present();
        canvas.present();
        loop_ctrl.end_frame();
    }
}