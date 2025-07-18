use std::{net::{SocketAddr, UdpSocket}, time::Duration};

use multiplayer_fps::{data::{self, default_addr, InputData, OutputData, PlayerData, PlayersData}, Loader};

pub fn connection(socket: &mut UdpSocket,server: SocketAddr,nickname: String) -> Result<(PlayerData,PlayersData,Loader), Box<dyn std::error::Error>> {
    let data = InputData::Connection(data::Connection {addr: default_addr(),nickname});
    let serialized = serde_json::to_string(&data)?;
    socket.send_to(serialized.as_bytes(), server)?;
    socket.set_read_timeout(Some(Duration::from_secs(10)))?;
    let data = OutputData::parse(socket)?;
    let (player,others,map) = match data {
        OutputData::Connecting(v) => v,
        OutputData::None => return Err("No response received from server within the timeout.".into()),
        _ =>return Err(format!("Unexpected response type: {:?}", data).into()),
    };
    Ok((player,others,map))
}