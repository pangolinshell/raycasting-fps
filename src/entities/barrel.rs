use sdl2::rect::Rect;

use crate::{entities::{entities::Entity, Player}, utils::vecs::from_direction, world::Map};

fn display(e: &mut Entity<'_> ,canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,from: Option<Player>, map: Option<&Map>) -> Result<(),String> {
    let from = match from {
        Some(p) => p,
        None => return Err(format!("from must be Some()")),
    };
    let map = match map {
        Some(m) => m,
        None => return Err(format!("map must be Some()")),
    };
    if !e.is_visible(map.clone(),from) {
        return Ok(());
    }
    let v_rect = canvas.viewport();
    let (screen_w, screen_h) = (v_rect.width(), v_rect.height());
    let (px, py) = from.position;
    let (dx, dy) = from_direction(from.direction);
    let fov_factor = from.fov_factor;
    let (dir_x,dir_y) = crate::utils::vecs::from_direction(from.direction);
    let plane_x = -dir_y * fov_factor;
    let plane_y =  dir_x * fov_factor;
    // Position relative à la caméra
    let sprite_x = e.position.0 - px;
    let sprite_y = e.position.1 - py;
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
    let texture = match e.textures.get("all") {
        Some(t) => t,
        None => return Err(format!("cannot find a texture named all")),
    };
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

pub fn new_barrel<'a>(x: f32,y: f32) -> Entity<'a> {
    let mut barrel  = Entity::new(x, y, 0.0);
    barrel.set_display(display);
    barrel
}