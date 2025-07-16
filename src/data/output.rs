use std::net::UdpSocket;

use crate::data::{Deny, Host, Hosts, Update};
pub use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize, Debug)]
#[serde(tag = "type",content = "data")] // <= ajoute un champ "type" dans le JSON
pub enum OutputData {
    Update(Update),
    AccessDeny(Deny),
    Connecting((Host,Hosts)),
    New(Host),
    Unknown,
    None,
}

impl OutputData {
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
                return Ok(Self::None);
            }
        };

        let (size, addr) = match opts {
            Some(values) => values,
            None => return Ok(Self::Unknown),
        };

        let data = String::from_utf8(buf[..size].to_vec())
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;

        let mut msg = serde_json::from_str::<Self>(&data).unwrap_or(Self::Unknown);
        match &mut msg {
            Self::Update(value) => value.addr = addr,
            _ => {},
        }
        Ok(msg)
    }
}