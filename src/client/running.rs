use std::error::Error;
use std::net::{SocketAddr};
use std::{net::UdpSocket};
use std::time::Duration;

use crate::data::{self, default_addr, InputData};

pub fn run(server_addr: &str,port: u32,name: String) -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind(format!("127.0.0.1:{}",port))?; // Port aléatoire local
    let server: SocketAddr = server_addr.parse()?;
    socket.set_read_timeout(Some(Duration::from_secs(2)))?; // Timeout lecture
    let data = InputData::Connection(data::Connection {addr: default_addr(),nickname: name});
    let serialized = serde_json::to_string(&data)?;
    socket.send_to(serialized.as_bytes(), server)?;
    loop {
        
    }
}