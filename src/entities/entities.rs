use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use sdl2::{rect::{FPoint, Rect}, render::Texture};

use crate::{display::Display, entities::Player, world::Map};

#[derive(Clone)]
pub struct Entity<'a> {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub look: Option<f32>,
    pub texture: Rc<RefCell<Texture<'a>>>,
    pub player: Rc<RefCell<Player>>, // << ici !
}


pub struct Entites<'a> {
    all: Vec<Entity<'a>>,
    map: Map<'a>,
    pub player: Rc<RefCell<Player>>
}

impl<'a> PartialEq for Entity<'a> {
    fn eq(&self, other: &Self) -> bool {
        let delta_a = delta((self.x,self.y), self.player.borrow().position);
        let delta_b = delta((other.x,other.y), other.player.borrow().position);
        delta_a == delta_b
    }
}

impl<'a> PartialOrd for Entity<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let delta_a = delta((self.x,self.y), self.player.borrow().position);
        let delta_b = delta((other.x,other.y), other.player.borrow().position);
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

impl<'a> Eq for Entity<'a> {}

impl<'a> Ord for Entity<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let delta_a = delta((self.x,self.y), self.player.borrow().position);
        let delta_b = delta((other.x,other.y), other.player.borrow().position);
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


impl<'a> Entites<'a> {
    pub fn init(map: Map<'a>,player: Rc<RefCell<Player>>) -> Self{
        Self {
            all: Vec::new(),
            map,
            player
        }
    }

    pub fn render(&mut self,canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(),String> {
        self.display(canvas)?;
        Ok(())
    }

    pub fn add(&mut self, entity: Entity<'a>) {
        self.all.push(entity.clone());
    }

    pub fn remove(&mut self,id: u32) {
        todo!("remove entity of id '{}'",id)
    }
}


impl<'a> Display<'a> for Entites<'a> {
    fn display(&mut self,canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(),String> {
        self.all.sort();
        let mut scan = self.all.clone();
        scan.retain(|e| e.is_visible(self.map.clone()));
        for mut e in scan {
            e.display(canvas)?;
        }
        Ok(())
    }

}

impl<'a> Entity<'a> {
    pub fn new(id: u32,pos: FPoint,texture:Rc<RefCell<Texture<'a>>>,camera: Rc<RefCell<Player>>) -> Self {
        Self { id, x: pos.x, y: pos.y, look: None, texture: texture.clone(), player:  camera}
    }

    // fn update_camera(&mut self,camera: &'a Player) {
    //     self.player = camera;
    // }

    pub fn is_visible(&self,map: Map) -> bool {
        self.is_in_fov() && !self.is_behind_a_wall(map)
    }

    pub fn is_behind_a_wall(&self, map: Map) -> bool {
        let (start_x, start_y) = self.player.borrow().position;
        let (end_x, end_y) = (self.x, self.y);

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

    pub fn is_in_fov(&self) -> bool {
        use crate::utils::vecs::from_direction;
    
        let camera = self.player.borrow(); // ← c'était self.player au lieu de self.pov
        let fov_factor = camera.fov_factor;
    
        let dir_vec = from_direction(camera.direction);
        let dx = self.x - camera.position.0;
        let dy = self.y - camera.position.1;
    
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
    
}

impl<'a> Display<'a> for Entity<'a> {
    fn display(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        let v_rect = canvas.viewport();
        let (screen_w, screen_h) = (v_rect.width(), v_rect.height());
        let (px, py) = self.player.borrow().position;
        let (dx, dy) = crate::utils::vecs::from_direction(self.player.borrow().direction);

        let fov_factor = self.player.borrow().fov_factor;
        let (dir_x,dir_y) = crate::utils::vecs::from_direction(self.player.borrow().direction);

        let plane_x = -dir_y * fov_factor;
        let plane_y =  dir_x * fov_factor;

        // Position relative à la caméra
        let sprite_x = self.x - px;
        let sprite_y = self.y - py;

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

        let texture = self.texture.clone();

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