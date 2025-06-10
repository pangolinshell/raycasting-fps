use sdl2::{pixels::Color, rect::{Rect}};

use crate::display;

#[derive(Debug,Clone,Copy)]
pub struct Ray {
    dist: f32,
    texture_code: u8,
    side: bool,
}

#[derive(Debug,Clone)]
pub struct Rays {
    rays: Vec<Ray>,
}

impl Ray {
    pub fn new(dist: f32,tcode: u8,side: bool) -> Self {
        Self { dist, texture_code: tcode, side}
    }
}

impl Rays {
    pub fn from(vec: Vec<Ray>) -> Self {
        Self { rays: vec }
    }
}

impl display::Display for Rays {
    fn display(&mut self,canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(),String> {
        let v_rect = canvas.viewport();
        let (_,h) = (v_rect.width(),v_rect.height());
        for (x,ray) in self.rays.iter().enumerate() {
            //Calculate height of line to draw on screen
            let line_height = (h as f32 /ray.dist) as i32;

            //calculate lowest and highest pixel to fill in current stripe
            let draw_start = (-line_height/2 + h as i32 /2).clamp(0, h as i32 - 1);

            let draw_end = ( line_height/2 + h as i32/2).clamp(0, h as i32 -1);

            let mut color = match ray.texture_code {
                1 => Color::RED,
                2 => Color::GREEN,
                3 => Color::BLUE,
                4 => Color::YELLOW,
                _ => Color::GRAY,
            };

            // if !ray.side {
            //     let (r,g,b) = color.rgb();
            //     color = Color::RGB(r/2, g/2, b/2);
            // }
            // let strt = Point::new(x as i32, draw_start);
            // let end = Point::new(x as i32, draw_end);
            // canvas.set_draw_color(color);
            // canvas.draw_line(strt, end)?;
            if ray.side {
               let (r,g,b) = color.rgb();
               color = Color::RGB(r/2, g/2, b/2);
            }
            let rect = Rect::new(
               x as i32,         // position X
               draw_start,       // position Y
               1,                // largeur (tu peux essayer 2 ou 3 aussi)
               (draw_end - draw_start) as u32,
            );
            canvas.set_draw_color(color);
            canvas.fill_rect(rect)?;
        }
        Ok(())
    }
}