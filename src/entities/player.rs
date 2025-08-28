use std::{net::SocketAddr, ops::Deref, str::FromStr};

use crate::{data::{default_addr, Status, Update}, entities::{entity::Movable, Entity}, world::Map};
use sdl2::rect::FPoint;
use serde::{Deserialize,Serialize};

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Player {
    // IP address and port of the Player.
    #[serde(skip, default = "default_addr")]
    pub addr: SocketAddr,

    /// Player's nickname or identifier.
    pub nickname: String, // Nicknames are uniques

    /// X coordinate of the Player's position.
    pub x: f32,

    /// Y coordinate of the Player's position.
    pub y: f32,

    /// Direction (angle or heading) of the Player.
    pub d: f32,

    /// Current status (Alive, Disconnected, etc.).
    pub status: Status,
    // pub data: PlayerData,
    // pub texture: Rc<Texture>

    pub texture: String,
}

impl Player {
    pub fn new<D: AsRef<str>>(name: String,xyd: (f32,f32,f32),texture: D) -> Self {
        Self { addr:default_addr(), nickname: name, x: xyd.0, y: xyd.1, d: xyd.2, status: Status::Alive, texture: texture.as_ref().to_string() }
    }

    pub fn update(&mut self, data: &Update) -> u8 {
        let mut modif_datas: u8 = 0;

        if let Some(x) = data.x {
            self.x = x;
            modif_datas += 1;
        }
        if let Some(y) = data.y {
            self.y = y;
            modif_datas += 1;
        }
        if let Some(d) = data.d {
            self.d = d;
            modif_datas += 1;
        }
        if let Some(status) = data.status {
            self.status = status;
            modif_datas += 1;
        }

        modif_datas
    }

    pub fn shoot(&self, map: &Map, players: &Players, hit_radius: f32) -> Option<Player> {
        let (mut x, mut y) = self.position();
        let step = 0.1; // précision du rayon
        let dx = self.d.cos() * step;
        let dy = self.d.sin() * step;

        loop {
            // Avancer le rayon
            x += dx;
            y += dy;

            // Vérifier collision avec mur
            if let Some(true) = map.is_wall(x.floor() as i32, y.floor() as i32) {
                return None;
            }

            // Vérifier joueurs
            for player in players.iter() {
                if player.nickname == self.nickname {
                    continue; // on ignore le tireur
                }

                let px = player.x;
                let py = player.y;
                let dist2 = (px - x).powi(2) + (py - y).powi(2);

                if dist2 <= hit_radius.powi(2) {
                    return Some(player.clone());
                }
            }

            // On fixe une distance max au rayon (évite boucle infinie si map vide)
            if ((x - self.x).powi(2) + (y - self.y).powi(2)).sqrt() > 1000.0 {
                return None;
            }
        }
    }
}

impl Movable for Player {
    fn direction(&self) -> f32 {
        self.d
    }

    fn position(&self) -> (f32,f32) {
        // (self.data.x,self.data.y)
        (self.x,self.y)
    }
}

impl<'a> Entity<'a> for Player {
    fn update(&mut self,_: Option<&mut super::Context>) -> Result<(),String> {
        Ok(())
    }

    fn entity_type(&self) -> super::entity::EntityType {
        super::entity::EntityType::Player
    }

    fn texture(&self) -> String {
        self.texture.clone()
    }
}

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct Players {
    pub players: Vec<Player>,
}

impl Deref for Players {
    type Target = Vec<Player>;
    fn deref(&self) -> &Self::Target {
        &self.players
    }
}

impl Players {
    pub fn new() -> Self {
        Self { players: Vec::new() }
    }

    pub fn into_coordinates(&self) -> Vec<FPoint>{
        let value = self.players.iter().map(|p| FPoint::new(p.x, p.y)).collect::<Vec<FPoint>>();
        value
    }

    // pub fn from(data_players: PlayersData,texture: Rc<Texture>) -> Self {
    //     let mut players = Vec::new();
    //     for player in data_players.players {
    //         players.push(Player::new(player, texture.clone()));
    //     }
    //     Self { players: players }
    // }

    pub fn update(&mut self, data: &Update) -> Option<u8> {
        let index = self.players.iter().position(|p| p.nickname == data.nickname);

        if let Some(index) = index {
            match data.status {
                Some(status) => {
                    match status {
                        crate::data::Status::Disconnecting => {
                            self.players.remove(index);
                            return None;
                        },
                        _ => (),
                    };
                },
                None => (),
            }
            match self.players.get_mut(index) {
                Some(p) => return Some(p.update(data)),
                None => return None,
            }
        }
        None
    }

    pub fn get_by_nickname<D: AsRef<str>>(&self, value: &D) -> Option<usize> {
        self.players.iter().position(|p| p.nickname == value.as_ref())
    }
    
    pub fn get_by_addr<A: IntoAddr>(&self, value: &A) -> Option<usize> {
        if let Ok(addr) = value.into_addr() {
            self.players.iter().position(|p| p.addr == addr)
        } else {
            None
        }
    }

    pub fn push(&mut self,player: Player) {
        self.players.push(player);
    }

    pub fn len(&self) -> usize {
        self.players.len()
    }

    pub fn remove(&mut self,index: usize) {
        self.players.remove(index);
    }
}

pub trait IntoAddr {
    fn into_addr(&self) -> Result<SocketAddr, ()>;
}

impl IntoAddr for SocketAddr {
    fn into_addr(&self) -> Result<SocketAddr, ()> {
        Ok(*self)
    }
}

impl IntoAddr for &str {
    fn into_addr(&self) -> Result<SocketAddr, ()> {
        SocketAddr::from_str(self).map_err(|_| ())
    }
}

impl IntoAddr for String {
    fn into_addr(&self) -> Result<SocketAddr, ()> {
        SocketAddr::from_str(self).map_err(|_| ())
    }
}
