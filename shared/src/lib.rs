use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerState {
    pub id: u32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub movement: Option<Direction>,
    pub angle_degrees: f32,
    pub alive: bool,
    pub inactive: bool,
    pub respawning: (bool, std::time::Duration),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Join(String),
    Move(Direction),
    Shoot,
    Respawn,
    Disconnect,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    Welcome(u32),
    GameState(Vec<PlayerState>),
    FullMapState {
        layout: Vec<Vec<u8>>,
        spawnpoints: Vec<map::SpawnPoint>,
        players: Vec<PlayerState>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

pub mod map {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Map {
        pub layout: Vec<Vec<u8>>,
        pub spawnpoints: Vec<SpawnPoint>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct SpawnPoint {
        pub x: usize,
        pub y: usize,
    }

    impl Map {
        pub fn is_walkable(&self, x: i32, y: i32) -> bool {
            if x < 0 || y < 0 {
                return false;
            }

            let xi = x as usize;
            let yi = y as usize;

            if yi >= self.layout.len() || xi >= self.layout[0].len() {
                return false;
            }

            self.layout[yi][xi] == 0
        }
    }
}
