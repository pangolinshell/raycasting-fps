use std::{cell::RefCell, rc::Rc};

use sdl2::{rect::FPoint, render::Texture};

/// Represents the type of an entity in the game.
/// 
/// - `Player`: A player-controlled entity.
/// - `Living`: A living, non-player entity (e.g., NPCs, monsters).
/// - `NoneLiving`: Non-living or static entities such as objects or pickups. These entities do not have a looking direction.
pub enum EntityType {
    Player,
    Living,
    NoneLiving, // For non-moving/non-living entity like objects, pickups, so they don't have a looking directions
}

/// Trait that defines common behavior for all entities in the game.
/// 
/// Implementors of this trait must provide methods for retrieving the entity's type, position, direction, field of view,
/// and for performing ray casting operations within a map.
pub trait Entity {
    /// Returns the type of the entity.
    fn get_type(&self) -> EntityType;

    /// Returns the position of the entity as an `FPoint`.
    fn position(&self) -> FPoint;

    /// Returns the direction the entity is facing, in radians.
    fn direction(&self) -> f32;

    /// Returns the field of view (FOV) of the entity, in radians.
    fn get_fov(&self) -> f32;

    fn texture(&self) -> Rc<RefCell<Texture>>;
}