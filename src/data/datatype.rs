use std::{net::UdpSocket, os::unix::net::SocketAddr};

use crate::server::data::{Update, Connection};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(tag = "type",content = "data")] // <= ajoute un champ "type" dans le JSON
pub enum DataType {
    Connection(Connection),
    Update(Update),
    Disconnection,
    Unknown,
    None,
}

impl DataType {
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
        let (size, addr) = match opts {
            Some(values) => values,
            None => return Ok(DataType::Unknown),
        };

        let data = String::from_utf8(buf[..size].to_vec())
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;

        let mut msg = serde_json::from_str::<DataType>(&data).unwrap_or(DataType::Unknown);
        match &mut msg {
            DataType::Update(value) => value.addr = addr,
            DataType::Connection(value) => value.addr = addr,
            _ => {},
        }
        Ok(msg)
    }
}