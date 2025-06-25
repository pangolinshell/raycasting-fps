use std::{cell::RefCell,rc::Rc};
use sdl2::{rect::Rect, render::Texture};
use crate::{display, entities::Player, world::Map};

#[derive(Clone)]
pub struct Ray<'a> {
    dist: f32,
    ray_dir_x: f32,
    ray_dir_y: f32,

    pos_x: f32,
    pos_y: f32,
    texture: Rc<RefCell<Texture<'a>>>,
    side: bool,
}

#[derive(Clone)]
pub struct Rays<'a>{
    rays: Vec<Ray<'a>>,
}

impl<'a> Ray<'a> {
    pub fn new(
        dist: f32,
        side: bool,
        ray_dir: (f32, f32),
        pos: (f32, f32),
        texture: Rc<RefCell<Texture<'a>>>,
    ) -> Self {
        Self {
            dist,
            side,
            ray_dir_x: ray_dir.0,
            ray_dir_y: ray_dir.1,
            pos_x: pos.0,
            pos_y: pos.1,
            texture: texture.clone(),
        }
    }
}

impl<'a> Rays<'a> {
    pub fn from(vec: Vec<Ray<'a>>) -> Self {
        Self { rays: vec }
    }
}

impl<'a> display::Display<'a> for Rays<'a>{
    fn display(&mut self,canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,from: Option<Player>,map: Option<&Map>) -> Result<(),String> {
        let v_rect = canvas.viewport();
        let (_,h) = (v_rect.width(),v_rect.height());
        for (x,ray) in self.rays.iter().enumerate() {
            //Calculate height of line to draw on screen
            let line_height = (h as f32 /ray.dist) as i32;

            //calculate lowest and highest pixel to fill in current stripe
            let draw_start = (-line_height/2 + h as i32 /2).clamp(0, h as i32 - 1);

            let draw_end = ( line_height/2 + h as i32/2).clamp(0, h as i32 -1);
            
            let perp_wall_dist = ray.dist;
            let mut wall_x = if !ray.side {
                ray.pos_y + perp_wall_dist * ray.ray_dir_y
            } else {
                ray.pos_x + perp_wall_dist * ray.ray_dir_x
            };
            wall_x -= wall_x.floor();

            let mut tex_x = (wall_x * 64.0) as i32;
            if!ray.side && ray.ray_dir_x > 0.0 {tex_x = 64 - tex_x - 1};
            if ray.side && ray.ray_dir_y < 0.0 {tex_x = 64 - tex_x - 1};
            let rect = Rect::new(
               x as i32,         // position X
               draw_start,       // position Y
               1,                // largeur (tu peux essayer 2 ou 3 aussi)
               (draw_end - draw_start) as u32,
            );

            let texture = &ray.texture;

            let mut tmp_texture = texture.borrow_mut();
            if ray.side {
                tmp_texture.set_color_mod(150, 150, 150);
            }
            canvas.copy(&tmp_texture, Rect::new(tex_x, 0, 1, 64), rect)?;
            tmp_texture.set_color_mod(255, 255, 255);
        }
        Ok(())
    }
}