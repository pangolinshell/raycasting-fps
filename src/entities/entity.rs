use std::rc::Rc;

use sdl2::{rect::FPoint, render::Texture};

use crate::{camera::Camera, entities::render_data::RenderData, world::Map, TextureManager};


pub trait Movable {
    fn position(&self) -> (f32,f32);
    fn direction(&self) -> f32;  
}

/// The `Entity` trait defines the common interface for all game entities.
/// 
/// # Type Parameters
/// - `'a`: Lifetime parameter for borrowed data, typically related to textures or context.
///
/// # Required Methods
/// - `position(&self) -> (f32, f32)`: Returns the (x, y) position of the entity.
/// - `direction(&self) -> f32`: Returns the facing direction (in radians or degrees) of the entity.
/// - `entity_type(&self) -> EntityType`: Returns the type of the entity.
/// - `texture(&self) -> Rc<Texture<'a>>`: Returns a reference-counted pointer to the entity's texture.
/// - `update(&mut self, ctx: Option<&mut Context<'a>>) -> Result<(), String>`: Updates the entity's state, possibly using a mutable context. Returns `Ok(())` on success or an error message on failure.
///
/// # Provided Methods
/// - `into_render(&self, camera: Camera, map: &Map<'a>) -> RenderData<'a>`: Converts the entity into render data for drawing, using the camera and map.
/// - `into_placement_data(&self) -> PlacementData`: Converts the entity into placement data, including position, direction, and type.
pub trait Entity<'a>: Movable {
    fn entity_type(&self) -> EntityType;

    fn texture(&self) -> String;
    fn update(&mut self,ctx: Option<&mut Context>) -> Result<(),String>;
    fn into_render<T>(&self, camera: Camera, map: &Map) -> RenderData {
        RenderData::new(camera, map.clone(), FPoint::from(self.position()), self.direction(), self.texture())
    }
    fn into_placement_data(&self) -> PlacementData {
        PlacementData { position: self.position(), direction: self.direction(), entity_type: self.entity_type()}
    }
}

pub enum EntityType {
    UnLiving,
    Living,
    Player,
}

/// Contains the placement information for an entity in the game world.
///
/// # Fields
/// - `position`: A tuple representing the (x, y) coordinates of the entity.
/// - `direction`: The facing direction of the entity, in radians.
/// - `entity_type`: The type of the entity, as defined by `EntityType`.
pub struct PlacementData {
    pub position: (f32,f32),
    pub direction: f32,
    pub entity_type: EntityType
}

/// Represents the context for an entity, containing references to the current map,
/// the Camera, and other entities' placement data.
///
/// # Fields
/// - `map`: The current game map context.
/// - `Camera`: The Camera entity associated with this context.
/// - `others`: A collection of placement data for other entities in the game.
pub struct Context {
    pub map: Map,
    pub camera: Camera,
    pub others:  Vec<PlacementData>,
}