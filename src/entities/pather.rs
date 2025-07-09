use std::rc::Rc;
use sdl2::{rect::FPoint, render::Texture};
use crate::{entities::{entity::Context, Entity}, utils::vecs::*};

/// A struct representing an entity that follows a path defined by a sequence of points ("way").
///
/// # Fields
/// - `position`: The current (x, y) position of the pather.
/// - `direction`: The current direction the pather is facing, in radians.
/// - `texture`: The texture used to render the pather.
/// - `speed`: The movement speed per update.
/// - `way`: The path to follow, represented as a vector of points.
/// - `target`: The index of the current target point in the path.
///
/// # Example
/// ```rust
/// let pather = Pather::new(0.0, 0.0, texture, 2.0);
/// ```
pub struct Pather<'a> {
    /// Current position of the pather.
    position: (f32, f32),
    /// Current direction (in radians).
    direction: f32,
    /// Texture used to render the pather.
    texture: Rc<Texture<'a>>,
    /// Movement speed per update.
    speed: f32,
    /// The path to follow: a vector of points.
    way: Vec<FPoint>,
    /// Index of the current target point in the path.
    target: usize,
}

impl<'a> Pather<'a> {
    pub fn new(x: f32,y:f32,defaut_texture: Rc<Texture<'a>>,speed: f32,) -> Self {
        Self { position: (x,y), direction: 0.0, texture: defaut_texture.clone(), speed: speed, way: Vec::new(), target: 0 }
    }

    pub fn path_from(&mut self,vec: Vec<FPoint>) {
        self.way = vec;
    }
}

/// Implementation of the `Entity` trait for the `Pather` struct.
/// 
/// The `Pather` entity represents a moving object that follows a predefined path (`way`).
/// It updates its position and direction based on its current target waypoint and speed.
/// 
/// # Methods
/// - `position`: Returns the current position of the entity as an (x, y) tuple.
/// - `entity_type`: Returns the type of the entity, which is `Living`.
/// - `direction`: Returns the current facing direction of the entity as a float (radians or degrees).
/// - `texture`: Returns a reference-counted pointer to the entity's texture.
/// - `update`: Updates the entity's position and direction, moving it towards the next waypoint.
///   When the entity reaches its current target waypoint (within `speed` distance), it advances to the next waypoint in the path.
///   The position and direction are recalculated accordingly.
impl<'a> Entity<'a> for Pather<'a> {
    fn position(&self) -> (f32,f32) {
        self.position
    }

    fn entity_type(&self) -> super::entity::EntityType {
        super::entity::EntityType::Living
    }

    fn direction(&self) -> f32 {
        self.direction
    }

    fn texture(&self) -> Rc<Texture<'a>> {
        self.texture.clone()
    }

    #[allow(unused_variables)]
    fn update(&mut self,ctx: Option<&mut Context<'a>>) -> Result<(),String> {
        if delta(self.position, (self.way[self.target].x,self.way[self.target].y)) <= self.speed {
            self.target = if self.target == self.way.len() - 1 {0} else {self.target + 1};
        }
        self.direction = direct_to(FPoint::from(self.position), self.way[self.target]);
        let new_position = go_toward(FPoint::from(self.position), self.way[self.target], self.speed);
        self.position = (new_position.x,new_position.y);
        Ok(())
    }
}