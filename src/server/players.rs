use std::time::{Instant};

pub enum Status {
    Connecting,
    Alive,
    Dead(Instant),
}

pub struct Player {
    nickname: String,

    x: f32,
    y: f32,
    d: f32,

    status: Status,
}