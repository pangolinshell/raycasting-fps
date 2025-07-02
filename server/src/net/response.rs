use serde::Serialize;
use shared::{Direction, Map};

#[derive(Serialize)]
pub struct ServerSnapshot {
    pub players: Vec<PlayerSnapshot>,
    pub map: MapSnapshot,
    pub events: Vec<GameEvent>,
}

#[derive(Serialize)]
pub struct PlayerSnapshot {
    pub id: u32,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub direction: Direction,
    pub alive: bool,
    pub respawning: bool,
}

#[derive(Serialize)]
pub struct MapSnapshot {
    pub layout: Vec<Vec<u8>>,              
    pub spawnpoints: Vec<(i32, i32)>,  
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum GameEvent {
    Player {
        player_id: u32,
        name: String,
        position: (i32, i32),
        facing : angle_degrees(Direction),
    },
    Move {
        player_id: u32,
        to: (i32, i32),
        direction: Direction, 
    },
    Shot {
        shooter_id: u32,
        from: (i32, i32),
        to: (i32, i32),
        hit_player: Option<u32>,
        hit_wall: Option<(i32, i32)>,
    },
    Death {
        player_id: u32,
        alive: bool,
        by_player_id: Option<u32>,
    },
    Respawn {
        player_id: u32,
        at: (i32, i32),
    },
}

