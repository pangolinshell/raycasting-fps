

use sdl2::{keyboard::Scancode};

use crate::{data::Update, rays::{Ray, Rays}, utils::{angles::degrees_to_rad, vecs::from_direction}, world::Map};

#[derive(Debug,Clone, Copy)]
pub struct Camera {
    pub position: (f32,f32),
    pub direction: f32,
    pub fov_factor: f32,
}

impl Camera {
    pub fn new(pos_x: f32,pos_y: f32,dir: f32) -> Self {
        Self { position: (pos_x,pos_y), direction: dir, fov_factor: 0.5 }
    }

    pub fn xyd(self) -> (f32,f32,f32) {
        let x = self.position.0;
        let y = self.position.1;
        let d = self.direction;
        (x,y,d)
    }

    pub fn update(&mut self,data: &Update) -> u8 {
        let mut modif_datas: u8 = 0;
        if let Some(x) = data.x {
            self.position.0 = x;
            modif_datas += 1;
        }
        if let Some(y) = data.y {
            self.position.1 = y;
            modif_datas += 1;
        }
        if let Some(d) = data.d {
            self.direction = d;
            modif_datas += 1;
        }
        // if let Some(status) = data.status {
        //     self.status = status;
        //     modif_datas += 1;
        // }

        modif_datas
    }
 
    pub fn cast_rays//<'a,'l,T>
    (&self, map: Map, w: u32
        // tm: &TextureManager<'l,T>
    ) -> Rays {
        let mut rays: Vec<Ray> = Vec::new();
        let (pos_x,pos_y) = self.position;
        let (dir_x,dir_y) = from_direction(self.direction);
        // let (plane_x,plane_y) =(0.0f32,0.66f32);
        let fov_factor = 0.5; // tu peux jouer avec ça 0.66 de (0.5 à 1.0 typiquement)
        let plane_x = -dir_y * fov_factor;
        let plane_y =  dir_x * fov_factor;

        for x in 0..w as usize {
            //calculate ray position and direction
            let camera_x: f32 = 2.0 * x as f32 / w as f32 - 1.0; //x-coordinate in camera space
            let raydir_x: f32 = dir_x + plane_x * camera_x;
            let raydir_y: f32 = dir_y + plane_y * camera_x;

            // wich box were in
            let (mut map_x,mut map_y) = (pos_x.floor() as i32,pos_y.floor() as i32);

            //length of ray from one x or y-side to next x or y-side
            let delta_dist_x = if raydir_x != 0.0 { (1.0 / raydir_x).abs() } else { f32::INFINITY };
            let delta_dist_y = if raydir_y != 0.0 { (1.0 / raydir_y).abs() } else { f32::INFINITY };

            let mut hit = false; //is it a wall hit?
            let mut side: bool = false; // ns (false) or ew (true)

                  //calculate step and initial sideDist
            let (step_x,mut side_dist_x) = if raydir_x < 0.0 {
              (-1 //what direction to step in x or y-direction (either +1 or -1)
                ,(pos_x - map_x as f32) * delta_dist_x) //length of ray from current position to next x or y-side
            }
            else
            {
              (1,(map_x as f32 + 1.0 - pos_x) * delta_dist_x)
            };

            let (step_y,mut side_dist_y) =  if raydir_y < 0.0 {
              (-1, (pos_y - map_y as f32) * delta_dist_y)
            }
            else
            {
              (1, (map_y as f32 + 1.0 - pos_y) * delta_dist_y)
            };

            let mut render_distance = 100;
            // perform DDA
            while hit == false && render_distance != 0{ 
                //jump to next map square, either in x-direction, or in y-direction
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = false;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y; 
                    side = true;
                }
                // check if ray has hit a wall
                let has_hit_wall = match map.is_wall(map_x, map_y) {
                    Some(v) => v,
                    None => false,
                };
                if has_hit_wall {
                    hit = true;
                }
                render_distance -= 1;
            }
            if render_distance <= 0 {
                continue;
            }
            // if side == false {
            //     perp_wall_dist = side_dist_x - delta_dist_x;
            // } else {
            //     perp_wall_dist = side_dist_x - delta_dist_x;
            // }
            let perp_wall_dist = if !side {
                side_dist_x - delta_dist_x
            } else {
                side_dist_y - delta_dist_y      
            };

            let tc = match map.get_tvalue(map_x, map_y) {
                Some(v) => v,
                None => 0,
            };
            let t_name = match map.texture_map.get(&tc) {
                Some(v) => v,
                None => &map.missing_texture,
            };
            // // correction fisheye
            // let corrected_dist = perp_wall_dist * (raydir_x * dir_x + raydir_y * dir_y);
            //     / ((raydir_x.powi(2) + raydir_y.powi(2)).sqrt() * (dir_x.powi(2) + dir_y.powi(2)).sqrt());
            // let t = match map.textures.get(&tc) {
            //     Some(t) => t.clone(),
            //     None =>  map.missing.clone(),
            // };
            
            rays.push(Ray::new(perp_wall_dist,side,(raydir_x,raydir_y),(pos_x,pos_y),t_name.clone()));
        }
        Rays::from(rays)
    }

    pub fn inputs(&mut self,event_pump: &mut sdl2::EventPump,delta_time: f32) {
        let keystate = event_pump.keyboard_state();
        let (px, py) = self.position;
        let speed = 6.0 * delta_time;
        let dir_angle = self.direction;
        let dir_x = dir_angle.cos();
        let dir_y = dir_angle.sin();
        let mut new_x = px;
        let mut new_y = py;
        let fov_factor = 0.5; // tu peux jouer avec ça (0.5 à 1.0 typiquement)
        let plane_x = -dir_y * fov_factor;
        let plane_y =  dir_x * fov_factor;

        // Avant / arrière = direction
        if keystate.is_scancode_pressed(Scancode::W) {
            new_x += dir_x * speed;
            new_y += dir_y * speed;
        }
        if keystate.is_scancode_pressed(Scancode::S) {
            new_x -= dir_x * speed;
            new_y -= dir_y * speed;
        }

        // Strafe droite / gauche = plan caméra
        if keystate.is_scancode_pressed(Scancode::D) {
            new_x += plane_x * speed;
            new_y += plane_y * speed;
        }
        if keystate.is_scancode_pressed(Scancode::A) {
            new_x -= plane_x * speed;
            new_y -= plane_y * speed;
        }

        if keystate.is_scancode_pressed(Scancode::E) {
            self.direction += degrees_to_rad(1.0) * speed * 20.0;
        }
        if keystate.is_scancode_pressed(Scancode::Q) {
            self.direction -= degrees_to_rad(1.0) * speed * 20.0;
        }

        self.position.0 = new_x;
        self.position.1 = new_y;
    }
}