use std::{net::{Ipv4Addr, SocketAddr, SocketAddrV4}, time::Instant};
use serde::{Deserialize, Serialize};


#[derive(Deserialize,Serialize,Debug,Clone,Copy)]
pub enum Status {
    Connecting,
    Alive,
    Dead(u64),
}


pub fn default_addr() -> SocketAddr {
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)) // 0.0.0.0:0
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct Update {
    #[serde(skip, default = "default_addr")]
    pub addr: SocketAddr,

    pub nickname: String,
    pub x: f32,
    pub y: f32,
    pub d: f32,
    pub status: Status,
}

impl Update {
    pub fn new(addr: SocketAddr,nickname: String,xyd: (f32,f32,f32)) -> Self {
        Self { addr,nickname, x: xyd.0, y: xyd.1, d: xyd.2, status: Status::Alive }
    }

    pub fn from_string(data: String,addr: SocketAddr) -> Result<Self, Box<dyn std::error::Error>> {
        let mut host: Self = serde_json::from_str(&data.as_str())?;
        host.addr = addr;
        Ok(host)
    }

    pub fn to_string(&self) -> Result<std::string::String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn is_addr(&self,addr: SocketAddr) -> bool {
        self.addr == addr
    }

    pub fn is_nickname(&self,nickname: String) -> bool {
        self.nickname == nickname
    }
}