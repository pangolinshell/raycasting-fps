use std::rc::Rc;
use sdl2::{rect::FPoint, render::Texture};
use crate::{entities::{entity::Context, Entity}, utils::vecs::*};

pub struct Straffer<'a> {
    position: (f32,f32),
    direction: f32,
    texture: Rc<Texture<'a>>,

    speed: f32,
    way: Vec<FPoint>,
    target: usize,
}

impl<'a> Straffer<'a> {
    pub fn new(x: f32,y:f32,defaut_texture: Rc<Texture<'a>>,speed: f32,) -> Self {
        Self { position: (x,y), direction: 0.0, texture: defaut_texture.clone(), speed: speed, way: Vec::new(), target: 0 }
    }

    pub fn path_from(&mut self,vec: Vec<FPoint>) {
        self.way = vec;
    }
}

impl<'a> Entity<'a> for Straffer<'a> {
    fn position(&self) -> (f32,f32) {
        self.position
    }

    fn entity_type(&self) -> super::entity::EntityType {
        super::entity::EntityType::Living
    }

    fn direction(&self) -> f32 {
        self.direction
    }

    fn texture(&self) -> Rc<Texture<'a>> {
        self.texture.clone()
    }

    fn update(&mut self,ctx: Option<&mut Context<'a>>) -> Result<(),String> {
        if delta(self.position, (self.way[self.target].x,self.way[self.target].y)) <= self.speed {
            self.target = if self.target == self.way.len() - 1 {0} else {self.target + 1};
        }
        self.direction = direct_to(FPoint::from(self.position), self.way[self.target]);
        let new_position = go_toward(FPoint::from(self.position), self.way[self.target], self.speed);
        self.position = (new_position.x,new_position.y);
        Ok(())
    }
}