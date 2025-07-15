use std::net::SocketAddr;

use crate::data::Status;

#[derive(Debug,Clone)]
pub struct Host {
    pub addr: SocketAddr,

    pub nickname: String,
    pub x: f32,
    pub y: f32,
    pub d: f32,
    pub status: Status,
}