
use serde::{Deserialize, Serialize};

use crate::{entities::{Player, Players}, world::Map};

#[derive(Deserialize,Serialize,Debug)]
pub struct Shoot {
    direction: f32,
    from: String,
    position: (f32,f32),
}

type Target = Player;

impl Shoot {
    /// Cast a ray from the shooting position
    /// return if a wall or nothing is touched
    /// return Some(Player) if the bullet touche a player
    pub fn cast(&self,map: Map,players: Players,hit_radius: f32) -> Option<Target> {
        
        todo!()
    }
}