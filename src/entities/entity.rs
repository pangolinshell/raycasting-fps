#![allow(static_mut_refs)]
use std::{rc::Rc,cell::RefCell};
use sdl2::rect::Rect;
use sdl2::render::Texture;
use crate::display::Display;
use crate::utils::vecs::from_direction;

use super::player::Player;


static mut PLAYER: Option<Rc<RefCell<Player>>> = None;

pub fn init_player(player: Rc<RefCell<Player>>) {
    unsafe {
        match PLAYER {
            Some(_) => panic!("player can only be initialized once"),
            None => (),
        };
       PLAYER = Some(player);
    }
}

fn get_player() -> Rc<RefCell<Player>> {
    unsafe {
        match PLAYER.clone() {
            Some(p) => p,
            None => panic!("must init player to use this function")
        }
    }
}

fn player_position() -> (f32,f32) {
    get_player().borrow().position
}

fn player_direction() -> f32 {
    get_player().borrow().direction
}

fn player_fov_factor() -> f32 {
    get_player().borrow().fov_factor
}

pub trait Entity {
    fn position(&self) -> (f32,f32);
    fn direction(&self) -> f32;
    fn texture(&self) -> Rc<RefCell<Texture<'_>>>;
}

impl<'a> Display<'a> for dyn Entity {
    fn display(&mut self,canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(),String> {
        let v_rect = canvas.viewport();
        let (screen_w, screen_h) = (v_rect.width(), v_rect.height());
        let (px, py) = player_position();
        let (dx, dy) = from_direction(player_direction());
        let fov_factor = player_fov_factor();
        let (dir_x,dir_y) = crate::utils::vecs::from_direction(player_direction());

        let plane_x = -dir_y * fov_factor;
        let plane_y =  dir_x * fov_factor;

        // Position relative à la caméra
        let sprite_x = self.position().0 - px;
        let sprite_y = self.position().1 - py;

        // Matrice de transformation inverse
        let inv_det = 1.0 / (plane_x * dy - dx * plane_y);
        let transform_x = inv_det * (dy * sprite_x - dx * sprite_y);
        let transform_y = inv_det * (-plane_y * sprite_x + plane_x * sprite_y);

        if transform_y <= 0.0 {
            return Ok(()); // derrière la caméra
        }

        let sprite_screen_x = ((screen_w as f32 / 2.0) * (1.0 + transform_x / transform_y)) as i32;

        let sprite_height = (screen_h as f32 / transform_y) as i32;
        let draw_start_y = (-sprite_height / 2 + screen_h as i32 / 2).clamp(0, screen_h as i32 - 1);
        let draw_end_y = (sprite_height / 2 + screen_h as i32 / 2).clamp(0, screen_h as i32 - 1);

        let sprite_width = sprite_height; // carré pour simplifier
        let draw_start_x = (-sprite_width / 2 + sprite_screen_x).clamp(0, screen_w as i32 - 1);
        let draw_end_x = (sprite_width / 2 + sprite_screen_x).clamp(0, screen_w as i32 - 1);

        let texture = self.texture();

        let src_rect = Rect::new(0, 0, 64, 64); // texture carrée

        let dst_rect = Rect::new(
            draw_start_x,
            draw_start_y,
            (draw_end_x - draw_start_x) as u32,
            (draw_end_y - draw_start_y) as u32,
        );

        let tex_ref = texture.borrow();
        canvas.copy(&*tex_ref, src_rect, dst_rect)?;
        Ok(())
    }
}

impl dyn Entity {
    pub fn distance_to_player(&self,player: &Player) -> f32 {
        let (p_x,p_y) = player.position;
        let (s_x,s_y) = self.position();

        f32::sqrt((p_x - s_x).powi(2) + (p_y - s_y).powi(2))
    }

    pub fn sort(list: &mut Vec<&Self>, player: &Player) {
        list.sort_by(|a, b| {
            let dist_a = a.distance_to_player(player);
            let dist_b = b.distance_to_player(player);
            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
        });
    }
}