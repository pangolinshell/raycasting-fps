use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize,Serialize)]
pub struct Level {
    pub layout: Vec<Vec<u8>>,
    spawnpoints: Vec<SpawnPoint>,
    pub textures: Textures,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct SpawnPoint {
    x: usize,
    y: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Textures {
    pub directory: String,
    pub tiles: std::collections::HashMap<u8, String>,
}

impl Level {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let s = String::from_utf8(buffer)?;
        let level: Level = serde_json::from_str(s.as_str())?;
        Ok(level)
    }
}