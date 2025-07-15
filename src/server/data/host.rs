use std::net::SocketAddr;

use crate::server::data::Entity;

#[derive(Debug,Clone)]
pub struct Host {
    pub addr: SocketAddr,
    pub data: Entity,
}

impl Host {
    pub fn new(data: Entity,addr: SocketAddr) -> Self {
        Self {
            addr,
            data
        }
    }
}