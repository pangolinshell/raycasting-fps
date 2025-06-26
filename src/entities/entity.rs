#![allow(static_mut_refs)]
use std::cmp::Ordering;
use std::{rc::Rc,cell::RefCell};
use sdl2::rect::Rect;
use sdl2::render::Texture;
use crate::utils::vecs::from_direction;
use crate::world::Map;

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

pub trait Entity<'a> {
    fn position(&self) -> (f32,f32);
    fn direction(&self) -> Option<f32>;
    fn texture(&self) -> Rc<Texture<'a>>;

    fn display(&mut self, map: &Map ,canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(),String> {
        if !self.is_visible(map.clone()) {
            return Ok(());
        }
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
        let draw_start_x = -sprite_width / 2 + sprite_screen_x; //.clamp(0, screen_w as i32 - 1);
        let draw_end_x = sprite_width / 2 + sprite_screen_x;//.clamp(0, screen_w as i32 - 1);

        let texture = self.texture();

        let src_rect = Rect::new(0, 0, 64, 64); // texture carrée

        let dst_rect = Rect::new(
            draw_start_x,
            draw_start_y,
            (draw_end_x - draw_start_x) as u32,
            (draw_end_y - draw_start_y) as u32,
        );

        let tex_ref = texture;
        canvas.copy(&*tex_ref, src_rect, dst_rect)?;
        Ok(())
    }

    fn distance_to_player(&self,player: &Player) -> f32 {
        let (p_x,p_y) = player.position;
        let (s_x,s_y) = self.position();

        f32::sqrt((p_x - s_x).powi(2) + (p_y - s_y).powi(2))
    }

    fn is_visible(&self,map: Map) -> bool {
        self.is_in_fov() && !self.is_behind_a_wall(map)
    }

    fn is_in_fov(&self) -> bool {
        use crate::utils::vecs::from_direction;
    
        let camera = get_player(); // ← c'était self.player au lieu de self.pov
        let fov_factor = camera.borrow().fov_factor;
    
        let dir_vec = from_direction(camera.borrow().direction);
        let dx = self.position().0 - camera.borrow().position.0;
        let dy = self.position().1 - camera.borrow().position.1;
    
        let dist = (dx * dx + dy * dy).sqrt();
        if dist == 0.0 {
            return true; // même position
        }
    
        let to_entity = (dx / dist, dy / dist);
        let dot = dir_vec.0 * to_entity.0 + dir_vec.1 * to_entity.1;
    
        let fov_rad = fov_factor * std::f32::consts::PI;
        let half_fov_cos = (fov_rad / 2.0).cos(); // ← cosinus de l'angle demi-FOV
    
        dot >= half_fov_cos
    }

    fn is_behind_a_wall(&self, map: Map) -> bool {
        let (start_x, start_y) = player_position();
        let (end_x, end_y) = (self.position().0, self.position().1);

        let dir_x = end_x - start_x;
        let dir_y = end_y - start_y;

        let ray_len = (dir_x * dir_x + dir_y * dir_y).sqrt();
        if ray_len == 0.0 {
            return false; // même position = pas de mur entre
        }

        let raydir_x = dir_x / ray_len;
        let raydir_y = dir_y / ray_len;

        let mut map_x = start_x.floor() as i32;
        let mut map_y = start_y.floor() as i32;

        let delta_dist_x = if raydir_x != 0.0 { (1.0 / raydir_x).abs() } else { f32::INFINITY };
        let delta_dist_y = if raydir_y != 0.0 { (1.0 / raydir_y).abs() } else { f32::INFINITY };

        let (step_x, mut side_dist_x) = if raydir_x < 0.0 {
            (-1, (start_x - map_x as f32) * delta_dist_x)
        } else {
            (1, (map_x as f32 + 1.0 - start_x) * delta_dist_x)
        };

        let (step_y, mut side_dist_y) = if raydir_y < 0.0 {
            (-1, (start_y - map_y as f32) * delta_dist_y)
        } else {
            (1, (map_y as f32 + 1.0 - start_y) * delta_dist_y)
        };

        let max_dist = ray_len;

        let mut dist_traveled = 0.0;

        // DDA loop
        while dist_traveled < max_dist {
            if side_dist_x < side_dist_y {
                map_x += step_x;
                dist_traveled = side_dist_x;
                side_dist_x += delta_dist_x;
            } else {
                map_y += step_y;
                dist_traveled = side_dist_y;
                side_dist_y += delta_dist_y;
            }

            if let Some(true) = map.is_wall(map_x, map_y) {
                return true; // mur entre joueur et entité
            }
        }

        false // pas de mur détecté entre les deux
    }
}

impl<'a> PartialEq for dyn Entity<'a> {
    fn eq(&self, other: &Self) -> bool {
        let delta_a = delta(self.position(), player_position());
        let delta_b = delta(other.position(), player_position());
        delta_a == delta_b
    }
}

impl<'a> PartialOrd for dyn Entity<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let delta_a = delta(self.position(), player_position());
        let delta_b = delta(other.position(), player_position());
        if delta_a < delta_b {
            Some(Ordering::Less)
        } else if delta_a > delta_b {
            Some(Ordering::Greater)
        } else if delta_a == delta_b {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

impl<'a> Eq for dyn Entity<'a> {}

impl<'a> Ord for dyn Entity<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let delta_a = delta(self.position(), player_position());
        let delta_b = delta(other.position(), player_position());
        if delta_a < delta_b {
            Ordering::Less
        } else if delta_a > delta_b {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
            
    }
}

fn delta(a: (f32, f32), b: (f32, f32)) -> f32 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    (dx * dx + dy * dy).sqrt()
}