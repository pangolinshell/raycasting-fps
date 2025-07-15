use std::net::SocketAddr;
use crate::data::{Connection, Status, Update};

#[derive(Debug,Clone)]
pub struct Host {
    pub addr: SocketAddr,

    pub nickname: String,
    pub x: f32,
    pub y: f32,
    pub d: f32,
    pub status: Status,
}

impl Host {
    pub fn init(value: Connection,xyd : (f32,f32,f32)) -> Self {
        Self { addr: value.addr, nickname: value.nickname, x: xyd.0, y: xyd.1, d: xyd.2, status: Status::Alive }
    }

    pub fn update(&mut self, data: Update) -> u8{
        let mut modif_datas: u8 = 0;
        if let Some(x) = data.x {
            self.x = x;
            modif_datas += 1;
        }
        if let Some(y) = data.y {
            self.y = y;
            modif_datas += 1;
        }
        if let Some(d) = data.d {
            self.d = d;
            modif_datas += 1;
        }
        if let Some(status) = data.status {
            self.status = status;
            modif_datas += 1;
        }
        modif_datas
    }
}