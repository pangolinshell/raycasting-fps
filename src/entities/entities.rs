use std::{collections::HashMap, rc::Rc};

use sdl2::{render::{Canvas, Texture}, video::Window};

use crate::{display::Display, entities::Player, world::Map};

pub struct Entity<'a> {
    pub position: (f32,f32),
    pub direction: f32,

    pub textures: HashMap<&'a str,Rc<Texture<'a>>>,
    behavior: Option<fn(&mut Self)>,
    display: Option<fn(&mut Self, &mut Canvas<Window>,from: Option<Player>,map: Option<&Map>) -> Result<(),String>>,
}

impl<'a> Entity<'a> {
    pub fn new(x: f32,y: f32, direction: f32) -> Self {
        Self {
            position: (x,y),
            direction,
            textures: HashMap::new(),
            behavior: None,
            display: None,
        }
    }

    pub fn insert_texture(&mut self, texture: Texture<'a>,name: &'a str) {
        self.textures.insert(name, Rc::new(texture));
    }

    pub fn set_display(&mut self, func: fn(&mut Self, &mut Canvas<Window>, from: Option<Player>,map: Option<&Map>) -> Result<(), String>) {
        self.display = Some(func);
    }

    pub fn is_visible(&self,map: Map,from: Player) -> bool {
        self.is_in_fov(from) && !self.is_behind_a_wall(map,from)
    }

    fn is_in_fov(&self,from: Player) -> bool {
        use crate::utils::vecs::from_direction;
    
        let camera = from; // ← c'était self.player au lieu de self.pov
        let fov_factor = camera.fov_factor;
    
        let dir_vec = from_direction(camera.direction);
        let dx = self.position.0 - camera.position.0;
        let dy = self.position.1 - camera.position.1;
    
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

    fn is_behind_a_wall(&self, map: Map, from: Player) -> bool {
        let (start_x, start_y) = from.position;
        let (end_x, end_y) = (self.position.0, self.position.1);

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

impl<'a> Display<'a> for Entity<'a> {
    fn display(&mut self, canvas: &mut Canvas<Window>, from: Option<Player>, map: Option<&Map>) -> Result<(), String> {
        match self.display {
            Some(func) => func(self, canvas,from,map),
            None => Err(format!("display behavior has not been defined")),
        }
    }
}