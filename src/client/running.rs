use std::error::Error;
use std::net::{UdpSocket,SocketAddr};
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::camera;
use crate::data::{self, default_addr, InputData, OutputData};
use crate::entities::{Player, Players};

pub fn run(server_addr: &str,port: u32,name: String) -> Result<(), Box<dyn Error>> {
    println!("starting client");
    let socket = UdpSocket::bind(format!("0.0.0.0:{}",port))?; // Port aléatoire local
    let server: SocketAddr = server_addr.parse()?;

    // socket.set_read_timeout(Some(Duration::from_secs(2)))?; // Timeout lecture

    let data = InputData::Connection(data::Connection {addr: default_addr(),nickname: name});
    let serialized = serde_json::to_string(&data)?;
    socket.send_to(serialized.as_bytes(), server)?;
    let mut players = Players::new();
    loop {
        let data = OutputData::parse(&socket)?;
        match data {
            OutputData::Connecting((me,others)) => {
                let camera = camera::Camera::new(me.x, me.y, me.d);
            }
            OutputData::None => println!("waiting"),
            _ => println!("not implemented yet"),
        }
    }
}

pub fn running(server_addr: &str,port: u32,name: String) -> Result<(), Box<dyn Error>> {
    println!("starting client");
    let socket = UdpSocket::bind(format!("0.0.0.0:{}",port))?; // Port aléatoire local
    socket.set_read_timeout(Some(Duration::from_secs(5)))?;

    let server: SocketAddr = server_addr.parse()?;
    let data = InputData::Connection(data::Connection {addr: default_addr(),nickname: name});
    let serialized = serde_json::to_string(&data)?;
    socket.send_to(serialized.as_bytes(), server)?;
    let mut players = Players::new();
    let data = OutputData::parse(&socket)?;
    let (camera) = match data {
        OutputData::Connecting((me,others)) => {
            camera::Camera::new(me.x, me.y, me.d)
        }
        OutputData::None => return Err(format!("timout on loggin").into()),
        _ => return Err(format!("unwanted data on loggin").into()),
    };
    
    Ok(())
}