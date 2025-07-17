use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Read};

/// Alias for a named identifier, typically used for assets (e.g. texture names).
type Name = String;

/// Alias for a file system path.
type Path = String;

/// Main structure representing a game map configuration.
///
/// Contains layout data, texture associations, and spawn points.
#[derive(Debug, Deserialize, Serialize)]
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
}

/// Represents a spawn point position on the map.
#[derive(Debug, Deserialize, Serialize)]
struct SpawnPoint {
    /// X coordinate in tile units.
    x: u8,

    /// Y coordinate in tile units.
    y: u8,
}

/// Structure holding paths to game resources like textures and fonts.
#[derive(Debug, Deserialize, Serialize)]
struct Resources {
    /// Base directory containing textures.
    textures_directory: Path,

    /// Base directory containing fonts.
    font_directory: Path,

    /// Mapping from texture name to relative or absolute file path.
    textures: HashMap<Name, Path>,

    /// Mapping from font name to relative or absolute file path.
    fonts: HashMap<Name, Path>,
}
