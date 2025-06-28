use std::rc::Rc;

use sdl2::{rect::FPoint, render::Texture};

use crate::{entities::{render_data::RenderData, Player}, world::Map};

pub trait Entity<'a> {
    fn position(&self) -> (f32,f32);
    fn direction(&self) -> f32;
    fn texture(&self) -> Rc<Texture<'a>>;
    fn update(&mut self,ctx: Option<Context<'a>>) -> Result<(),String>;
    fn into_render(&self, camera: Player, map: &Map<'a>) -> RenderData<'a> {
        RenderData::new(camera, map.clone(), FPoint::from(self.position()), self.direction(), self.texture())
    }
}

pub struct Context<'a> {
    pub map: &'a Map<'a>,
    pub player: &'a Player,
    pub others: Vec<Box<dyn Entity<'a>>>,
}