use std::{net::SocketAddr, time::Instant};
use serde::{Deserialize, Serialize};


#[derive(Deserialize,Serialize,Debug,Clone,Copy)]
pub enum Status {
    Connecting,
    Alive,
    Dead(u64),
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct Entity {
    #[serde(skip)]
    pub addr: String,
    pub nickname: String,
    pub x: f32,
    pub y: f32,
    pub d: f32,
    pub status: Status,
}

impl Entity {
    pub fn new(addr: SocketAddr,nickname: String,xyd: (f32,f32,f32)) -> Self {
        Self { addr: addr.to_string(), nickname, x: xyd.0, y: xyd.1, d: xyd.2, status: Status::Alive }
    }

    pub fn from_string(data: String,addr: SocketAddr) -> Result<Self, Box<dyn std::error::Error>> {
        let mut v: Self = serde_json::from_str(&data.as_str())?;
        v.addr = addr.to_string();
        Ok(v)
    }

    pub fn to_string(&self) -> Result<std::string::String, serde_json::Error> {
        serde_json::to_string(self)
    }
}