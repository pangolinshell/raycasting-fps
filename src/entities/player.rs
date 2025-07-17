use std::rc::Rc;

use sdl2::render::Texture;

use crate::{data::{Host, Hosts, Update}, entities::Entity};

pub struct Player<'a> {
    pub data: Host,

    pub texture: Rc<Texture<'a>>
}

impl<'a> Player<'a> {
    pub fn new(host: Host,texture: Rc<Texture<'a>>) -> Self {
        Self { data: host, texture: texture.clone() }
    }

    pub fn update(&mut self,data: Update) -> u8 {
        self.data.update(data)
    }
}

impl<'a> Entity<'a> for Player<'a> {
    fn direction(&self) -> f32 {
        self.data.d
    }

    fn position(&self) -> (f32,f32) {
        (self.data.x,self.data.y)
    }

    
    fn update(&mut self,ctx: Option<&mut super::Context<'a>>) -> Result<(),String> {
        Ok(())
    }

    fn entity_type(&self) -> super::entity::EntityType {
        super::entity::EntityType::Player
    }

    fn texture(&self) -> std::rc::Rc<sdl2::render::Texture<'a>> {
        self.texture.clone()
    }
}

pub struct Players<'a> {
    pub players: Vec<Player<'a>>,
}

impl<'a> Players<'a> {
    pub fn new() -> Self {
        Self { players: Vec::new() }
    }

    pub fn from(hosts: Hosts,texture: Rc<Texture<'a>>) -> Self {
        let mut players = Vec::new();
        for host in hosts.hosts {
            players.push(Player::new(host, texture.clone()));
        }
        Self { players: players }
    }

    pub fn update(&mut self,data: Update) -> Option<u8> {
        for p in self.players.iter_mut() {
            if p.data.nickname == data.nickname {
                return Some(p.data.update(data));
            }
        }
        None
    }
}