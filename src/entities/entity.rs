use std::rc::Rc;

use sdl2::{rect::FPoint, render::Texture};

use crate::{entities::{render_data::RenderData, Player}, world::Map};


pub trait Entity<'a> {
    fn position(&self) -> (f32,f32);
    fn direction(&self) -> f32;
    fn entity_type(&self) -> EntityType;

    fn texture(&self) -> Rc<Texture<'a>>;
    fn update(&mut self,ctx: Option<&mut Context<'a>>) -> Result<(),String>;
    fn into_render(&self, camera: Player, map: &Map<'a>) -> RenderData<'a> {
        RenderData::new(camera, map.clone(), FPoint::from(self.position()), self.direction(), self.texture())
    }
    fn into_placement_data(&self) -> PlacementData {
        PlacementData { position: self.position(), direction: self.direction(), entity_type: self.entity_type()}
    }
}

pub enum EntityType {
    UnLiving,
    Living,
    Player,
}

pub struct PlacementData {
    pub position: (f32,f32),
    pub direction: f32,
    pub entity_type: EntityType
}

pub struct Context<'a> {
    pub map: Map<'a>,
    pub player: Player,
    pub others:  Vec<PlacementData>,
}