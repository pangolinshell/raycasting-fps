use std::error::Error;
use std::net::{UdpSocket,SocketAddr};
use std::time::Duration;

use crate::data::{self, default_addr, InputData, OutputData};

pub fn run(server_addr: &str,port: u32,name: String) -> Result<(), Box<dyn Error>> {
    println!("starting client");
    let socket = UdpSocket::bind(format!("0.0.0.0:{}",port))?; // Port aléatoire local
    let server: SocketAddr = server_addr.parse()?;

    socket.set_read_timeout(Some(Duration::from_secs(2)))?; // Timeout lecture

    let data = InputData::Connection(data::Connection {addr: default_addr(),nickname: name});
    let serialized = serde_json::to_string(&data)?;
    socket.send_to(serialized.as_bytes(), server)?;
    loop {
        let data = OutputData::parse(&socket)?;
        match data {
            OutputData::Connecting((me,others)) => {
                println!("got it!")
            }
            OutputData::None => println!("waiting"),
            _ => println!("not what i am waiting for"),
        }
    }
}