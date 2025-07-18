use std::net::UdpSocket;

use crate::{data::{Deny, PlayerData, PlayersData, Update}, Loader};
pub use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize, Debug)]
#[serde(tag = "type",content = "data")] // <= ajoute un champ "type" dans le JSON
pub enum OutputData {
    Update(Update),
    AccessDeny(Deny),
    Connecting((PlayerData,PlayersData,Loader)),
    New(PlayerData),
    Unknown,
    None,
}

impl OutputData {
    pub fn parse(socket: &UdpSocket) -> Result<Self, Box<std::io::Error>> {
        // init a buffer of 1024 bytes
        let mut buf = [0; 4048];

        // data reception (non-blocking)
        let opts = match socket.recv_from(&mut buf) {
            Ok(v) => v,
            Err(e) => {
                if !matches!(e.kind(), std::io::ErrorKind::WouldBlock) {
                    return Err(Box::new(e));
                }
                return Ok(Self::None);
            }
        };

        // let (size, addr) = match opts {
        //     Some(values) => values,
        //     None => return Ok(Self::Unknown),
        // };

        let data = String::from_utf8(buf[..opts.0].to_vec())
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;

        let mut msg = serde_json::from_str::<OutputData>(&data).unwrap_or(OutputData::Unknown);
        // dbg!(data);
        match &mut msg {
            Self::Update(value) => value.addr = opts.1,
            _ => {},
        }
        Ok(msg)
    }
}