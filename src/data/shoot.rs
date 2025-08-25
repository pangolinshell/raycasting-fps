
use serde::{Deserialize, Serialize};
#[derive(Deserialize,Serialize,Debug)]
pub struct Shoot {
    direction: f32,
    position: (f32,f32),
}