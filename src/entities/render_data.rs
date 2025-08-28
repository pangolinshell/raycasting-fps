#![allow(unused,)]

use std::{fmt::format, rc::Rc};
use sdl2::{rect::{FPoint, Rect}, render::Texture};
use crate::{TextureManager, display::Display, rays::Rays, utils::vecs::*, world::Map};
use std::cmp::Ordering;
use crate::camera::Camera;

/// `RenderData` encapsulates all data required to render an entity (e.g., sprite)
/// in a pseudo-3D environment using raycasting techniques.
///
/// # Fields
/// - `camera`: The Camera struct representing the viewpoint (position, direction, FOV).
/// - `map`: The game map, used to determine visibility and wall collisions.
/// - `position`: The (x, y) coordinates of the entity.
/// - `_direction`: Direction the entity is facing (currently unused).
/// - `texture`: The texture used to render the entity.
///
/// # Methods
/// - `new`: Constructs a new `RenderData`.
/// - `display`: Renders the entity on the SDL2 canvas if visible.
/// - `distance_to_player`: Calculates Euclidean distance to the Camera.
/// - `is_visible`: Checks if the entity is visible (within FOV and not obstructed).
/// - `is_in_fov`: Checks if the entity lies within the Camera's field of view.
/// - `is_behind_a_wall`: Uses DDA raycasting to check for wall obstruction.
///
/// # Traits
/// Implements `PartialEq`, `PartialOrd`, `Eq`, and `Ord` to allow sorting
/// entities by distance (useful for painter's algorithm).
#[derive(Clone)]
#[allow(unused)]
pub struct RenderData {
    /// The Camera's viewpoint (position, direction, FOV)
    camera: Camera,
    /// The game map used for collision and visibility checks
    map: Map,
    /// The entity's position in the world
    position: (f32,f32),
    /// The direction the entity is facing (unused)
    direction: f32,
    /// The texture used to draw the entity
    texture: String,

    rays: Rays
}

impl RenderData {
    /// Creates a new `RenderData` instance
    pub fn new(camera: Camera, map: Map, position: FPoint, direction: f32, texture: String,rays:Rays) -> Self {
        Self {
            camera,
            map: map.clone(),
            position: (position.x, position.y),
            direction: direction,
            texture,
            rays,
        }
    }

    /// Computes Euclidean distance from the entity to the given Camera.
    fn distance_to_player(&self, camera: &Camera) -> f32 {
        let (p_x, p_y) = camera.position;
        let (s_x, s_y) = self.position;
        f32::sqrt((p_x - s_x).powi(2) + (p_y - s_y).powi(2))
    }

    /// Determines whether the entity is visible to the Camera.
    fn is_visible(&self, map: Map) -> bool {
        self.is_in_fov() && !self.is_behind_a_wall(map)
    }

    /// Checks if the entity is within the field of view of the camera.
    fn is_in_fov(&self) -> bool {
        let camera = self.camera;
        let fov_factor = camera.fov_factor;

        let dir_vec = from_direction(camera.direction);
        let dx = self.position.0 - camera.position.0;
        let dy = self.position.1 - camera.position.1;

        let dist = (dx * dx + dy * dy).sqrt();
        if dist == 0.0 {
            return true; // Same position
        }

        let to_entity = (dx / dist, dy / dist);
        let dot = dir_vec.0 * to_entity.0 + dir_vec.1 * to_entity.1;

        let fov_rad = fov_factor * std::f32::consts::PI;
        let half_fov_cos = (fov_rad / 2.0).cos();

        dot >= half_fov_cos
    }

    /// Uses DDA raycasting to check if a wall is between the Camera and the entity.
    fn is_behind_a_wall(&self, map: Map) -> bool {
        let (start_x, start_y) = self.camera.position;
        let (end_x, end_y) = (self.position.0, self.position.1);

        let dir_x = end_x - start_x;
        let dir_y = end_y - start_y;

        let ray_len = (dir_x * dir_x + dir_y * dir_y).sqrt();
        if ray_len == 0.0 {
            return false; // Same position
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
                return true; // Wall detected
            }
        }

        false // No wall detected
    }
}

// === Trait Implementations for Sorting ===

impl PartialEq for RenderData {
    /// Compares distances to the camera for equality.
    fn eq(&self, other: &Self) -> bool {
        let delta_a = delta(self.position, self.camera.position);
        let delta_b = delta(other.position, self.camera.position);
        delta_a == delta_b
    }
}

impl PartialOrd for RenderData {
    /// Compares distances to the camera for ordering.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let delta_a = delta(self.position, self.camera.position);
        let delta_b = delta(other.position, self.camera.position);
        delta_a.partial_cmp(&delta_b)
    }
}

impl Eq for RenderData {}

impl Ord for RenderData {
    /// Fully orders entities by distance to the camera.
    fn cmp(&self, other: &Self) -> Ordering {
        let delta_a = delta(self.position, self.camera.position);
        let delta_b = delta(other.position, self.camera.position);
        delta_a.partial_cmp(&delta_b).unwrap_or(Ordering::Equal)
    }
}

impl Display for RenderData {
    fn display<T>(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture_manager: Option<&TextureManager<T>>,
    ) -> Result<(), String> {
        let texture_manager = match texture_manager {
            Some(tm) => tm,
            None => return Err(format!("must add some texutres")),
        };
        // --- Projection caméra ---
        let v_rect = canvas.viewport();
        let (screen_w, screen_h) = (v_rect.width(), v_rect.height());
        let (px, py) = self.camera.position;
        let (dx, dy) = from_direction(self.camera.direction);
        let fov_factor = self.camera.fov_factor;

        let (dir_x, dir_y) = from_direction(self.camera.direction);
        let plane_x = -dir_y * fov_factor;
        let plane_y = dir_x * fov_factor;

        // Position relative entité -> caméra
        let sprite_x = self.position.0 - px;
        let sprite_y = self.position.1 - py;

        // Transformation caméra
        let inv_det = 1.0 / (plane_x * dy - dx * plane_y);
        let transform_x = inv_det * (dy * sprite_x - dx * sprite_y);
        let transform_y = inv_det * (-plane_y * sprite_x + plane_x * sprite_y);

        if transform_y <= 0.0 {
            return Ok(()); // derrière caméra
        }

        // Projection écran
        let sprite_screen_x =
            ((screen_w as f32 / 2.0) * (1.0 + transform_x / transform_y)) as i32;

        let sprite_height = (screen_h as f32 / transform_y) as i32;
        let draw_start_y =
            (-sprite_height / 2 + screen_h as i32 / 2).clamp(0, screen_h as i32 - 1);
        let draw_end_y =
            (sprite_height / 2 + screen_h as i32 / 2).clamp(0, screen_h as i32 - 1);

        let sprite_width = sprite_height;
        let draw_start_x = -sprite_width / 2 + sprite_screen_x;
        let draw_end_x = sprite_width / 2 + sprite_screen_x;

        // Récup texture
        let texture = match texture_manager.get(&self.texture) {
            Some(texture) => texture,
            None => return Err(format!("texture \"{}\" not found", self.texture)),
        };

        // --- Rendu colonne par colonne avec Rays comme z-buffer ---
        for stripe in draw_start_x..draw_end_x {
            if stripe >= 0 && stripe < screen_w as i32 {
                // Vérifie si sprite est devant le mur (z-buffer check)
                if transform_y < self.rays.rays[stripe as usize].dist {
                    let dst_rect = Rect::new(
                        stripe,
                        draw_start_y,
                        1, // une colonne de large
                        (draw_end_y - draw_start_y) as u32,
                    );

                    let tex_x = ((stripe - draw_start_x) * 64 / sprite_width) as i32;
                    let src_rect = Rect::new(tex_x, 0, 1, 64);

                    canvas.copy(&*texture, src_rect, dst_rect)?;
                }
            }
        }

        Ok(())
    }
}
