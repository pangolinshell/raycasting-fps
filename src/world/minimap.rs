use sdl2::{pixels::Color, rect::FPoint};

use crate::{ display};

use super::Map;

const DEFAULT_WALL_SIZE: u8 = 10;

pub struct Minimap {
    map: Map,
    target: FPoint,

    wall_size: u8,

    wall_color: Color,
    bg_color: Color,
    target_color: Option<Color>
}

impl Minimap {
    pub fn new(map: &Map,target: &FPoint, wall_color: Color, bg_color: Color) -> Self {
        Self { map: map.clone(), target: target.clone(), wall_size: DEFAULT_WALL_SIZE, wall_color: wall_color.clone(), bg_color: bg_color.clone(), target_color: None }
    }

    pub fn set_target_pinpoint(&mut self,color: Option<Color>) {
        self.target_color = color
    }

    pub fn set_wall_size(&mut self,size: usize) {
        self.wall_size = size as u8
    }
}

impl display::Display for Minimap {
    fn display<'l,T>(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        _textures: Option<&crate::TextureManager<'l,T>>
    ) -> Result<(),String> {

        let viewport = canvas.viewport();
        let vp_w = viewport.width() as i32;
        let vp_h = viewport.height() as i32;

        // fond
        canvas.set_draw_color(self.bg_color);
        // canvas.clear();

        // position de la target en pixels
        let target_px_x = (self.target.x * self.wall_size as f32) as i32;
        let target_px_y = (self.target.y * self.wall_size as f32) as i32;

        // décalage pour centrer la target
        let offset_x = vp_w / 2 - target_px_x;
        let offset_y = vp_h / 2 - target_px_y;

        // murs
        for tile in &self.map.layout {
            if tile.is_wall() {
                let rect = sdl2::rect::Rect::new(
                    tile.pos.x * self.wall_size as i32 + offset_x,
                    tile.pos.y * self.wall_size as i32 + offset_y,
                    self.wall_size as u32,
                    self.wall_size as u32
                );
                canvas.set_draw_color(self.wall_color);
                canvas.fill_rect(rect)?;
            }
        }

        // target au centre
        if let Some(color) = self.target_color {
            let rect = sdl2::rect::Rect::new(
                vp_w / 2 - (self.wall_size as i32 / 4),
                vp_h / 2 - (self.wall_size as i32 / 4),
                (self.wall_size / 2) as u32,
                (self.wall_size / 2) as u32
            );
            canvas.set_draw_color(color);
            canvas.fill_rect(rect)?;
        }

        Ok(())
    }
}
