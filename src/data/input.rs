use std::{net::UdpSocket, net::SocketAddr};

use crate::data::{default_addr, Connection, Shoot, Update};
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize, Debug)]
#[serde(tag = "type",content = "data")] // <= ajoute un champ "type" dans le JSON
pub enum InputData {
    Connection(Connection),
    Update(Update),
    Disconnection {
        #[serde(skip,default = "default_addr")]
        addr: SocketAddr,
    },
    Shoot(Shoot),
    Unknown, // Malformed request
    None, // nothing recieved
}

impl InputData {
    pub fn parse(socket: &UdpSocket) -> Result<Self, Box<std::io::Error>> {
        // init a buffer of 1024 bytes
        let mut buf = [0; 1024];

        // data reception (non-blocking)
        let opts = match socket.recv_from(&mut buf) {
            Ok(v) => Some(v),
            Err(e) => {
                if !matches!(e.kind(), std::io::ErrorKind::WouldBlock) {
                    return Err(Box::new(e));
                }
                None
            }
        };

        // 
        let (size, socket_addr) = match opts {
            Some(values) => values,
            None => return Ok(InputData::None),
        };

        let data = String::from_utf8(buf[..size].to_vec())
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;

        let mut msg = serde_json::from_str::<InputData>(&data).unwrap_or(InputData::Unknown);
        match &mut msg {
            InputData::Update(value) => value.addr = socket_addr,
            InputData::Connection(value) => value.addr = socket_addr,
            InputData::Disconnection { addr } => *addr = socket_addr,
            _ => {},
        }
        Ok(msg)
    }
}