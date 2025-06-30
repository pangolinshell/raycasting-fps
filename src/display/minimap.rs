use crate::{player::Player, world::{Map, TileType}};
use std::rc::Rc;
use sdl2::{pixels::Color, rect::{Point, Rect}};

use super::display::Display;

pub struct Minimap<'a> {
    map: Rc<Map<'a>>,
    display: Rect,
    wall_size: u32,
}

impl<'a> Minimap<'a> {
    pub fn new(map: Map<'a>, wall_size: u32, size: u32, position: Point) -> Self {
        Self { map: Rc::new(map), display: Rect::new(position.x, position.y, size, size), wall_size }
    }
}

impl<'a> Display<'a> for Minimap<'a> {
    #[allow(unused)]
    fn display(&mut self,canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, from: Option<Player>, map: Option<&Map>) -> Result<(),String> {
        let vp = canvas.viewport();
        canvas.set_viewport(self.display);
        canvas.set_draw_color(Color::GRAY);
        for tile in self.map.layout.iter() {
            match tile.t_type {
                TileType::Wall(_) => (),
                TileType::Void => continue,
            }
            let r = Rect::new(
               tile.pos.x * self.wall_size as i32,
               tile.pos.y * self.wall_size as i32,
               self.wall_size,
               self.wall_size
           );
           canvas.fill_rect(r)?;
        }       
        canvas.set_viewport(vp);
        Ok(())
    }
}