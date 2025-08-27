use sdl2::rect::Point;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Read};

use crate::{resources::FontDetails, world::{Map, Tile}};

/// Alias for a named identifier, typically used for assets (e.g. texture names).
type Name = String;

/// Alias for a file system path.
type Path = String;

/// Alias for the font size
type Size = u16;

/// Main structure representing a game map configuration.
///
/// Contains layout data, texture associations, and spawn points.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Loader {
    /// 2D grid representing the map layout.
    /// Each `u8` corresponds to a tile ID.
    layout: Vec<Vec<u8>>,

    /// Default texture name to use as a placeholder
    /// if a tile ID has no matching texture.
    placeholder: Name,

    /// Maps tile IDs (`u8`) to texture names.
    textures_bindings: HashMap<u8, Name>,

    /// List of spawn points on the map.
    spawnpoints: Vec<SpawnPoint>,

    resources: Resources,
}

/// Represents a spawn point position on the map.
#[derive(Debug, Deserialize, Serialize, Clone)]
struct SpawnPoint {
    /// X coordinate in tile units.
    x: u8,

    /// Y coordinate in tile units.
    y: u8,
}

/// Structure holding paths to game resources like textures and fonts.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Resources {
    /// Base directory containing textures.
    textures_directory: Path,

    /// Base directory containing fonts.
    font_directory: Path,

    /// Mapping from texture name to relative or absolute file path.
    textures: HashMap<Name, Path>,

    /// Mapping from font name to relative or absolute file path.
    fonts: HashMap<Name, Fonts>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Fonts {
    pub path: Path,
    pub size: u16,
}

impl Resources {
    /// Builds a map of texture names to their full paths, combining each entry in
    /// `self.textures` with the `textures_directory`.
    ///
    /// # Returns
    /// - `Ok(HashMap<Name, Path>)` with full paths for each texture.
    /// - `Err` if a duplicate key is found.
    pub fn textures(&self) -> Result<HashMap<Name, Path>, Box<dyn std::error::Error>> {
        let mut t_map: HashMap<Name, Path> = HashMap::new();
        let t_dir = if !self.textures_directory.ends_with("/") {
            format!("{}/", self.textures_directory)
        } else {
            self.textures_directory.clone()
        };

        for (name, path) in &self.textures {
            match t_map.insert(name.clone(), format!("{}{}", t_dir, path)) {
                Some(_) => return Err(format!("key {} is in double", name.clone()).into()),
                None => (),
            };
        }

        Ok(t_map)
    }

    /// Builds a map of font names to their full paths, combining each entry in
    /// `self.fonts` with the `font_directory`.
    ///
    /// # Returns
    /// - `Ok(HashMap<Name, Path>)` with full paths for each font.
    /// - `Err` if a duplicate key is found.
    pub fn fonts(&self) -> Result<HashMap<Name, (Path,Size)>, Box<dyn std::error::Error>> {
        let mut t_map: HashMap<Name, (Path,Size)> = HashMap::new();
        let t_dir = if !self.font_directory.ends_with("/") {
            format!("{}/", self.font_directory)
        } else {
            self.font_directory.clone()
        };

        for (name, font) in &self.fonts {
            match t_map.insert(name.clone(), (format!("{}{}", t_dir, font.path),font.size)) {
                Some(_) => return Err(format!("key {} is in double", name.clone()).into()),
                None => (),
            };
        }

        Ok(t_map)
    }
}

impl Loader {
    /// Loads a `Loader` (map configuration) from a JSON file.
    ///
    /// # Arguments
    /// - `path`: Path to the JSON file.
    ///
    /// # Returns
    /// - `Ok(Loader)` if parsing succeeds.
    /// - `Err` if the file can't be read or parsed.
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let s = String::from_utf8(buffer)?;
        let level: Self = serde_json::from_str(s.as_str())?;
        Ok(level)
    }

    pub fn get_resources(&self) -> &Resources {
        &self.resources
    }
}

impl<'a> From<&'a Fonts> for FontDetails {
    fn from(details: &'a Fonts) -> Self {
        Self {
            path: details.path.clone(),
            size: details.size,
        }
    }
}

 impl<'a> From<&Loader> for Map {
    fn from(value: &Loader) -> Self {
        let mut layout: Vec<Tile> = Vec::new();
        let t_map: HashMap<u8,String> = value.textures_bindings.clone();
        for (y,line) in value.layout.iter().enumerate() {
            for (x,value) in line.iter().enumerate() {
                layout.push(Tile::new(Point::new(x as i32, y as i32), *value));
            } 
        }
        let mut spawn_points = Vec::new();
        for spawn_point in &value.spawnpoints {
            match layout.iter().find(|t| {
                (spawn_point.x as i32,spawn_point.y as i32) == (t.pos.x,t.pos.y)
            }) {
                Some(t) => spawn_points.push(*t), 
                None => (),
            }
        }
        Map::new(layout, t_map, value.placeholder.clone(),spawn_points)
    }
}