use std::net::{Ipv4Addr, UdpSocket};
use std::thread;

use crate::data::{InputData, Hosts};
use crate::server::logic::{connection, update};

const DEFAULT_MAX_HOSTS: u8 = 4;

type Error = Box<dyn std::error::Error>;

#[derive(Clone)]
/// Represents a server instance with configuration parameters.
/// All modification to the instance must be made before running.
/// All modification made after will be applied on restart
///
/// # Fields
/// - `port`: The network port on which the server instance listens.
/// - `frequency`: The tick/update frequency of the server instance.
/// - `max_hosts`: The maximum number of hosts allowed to connect (default is 4).
pub struct Instance {
    port: u32,
    frequency: u32,
    max_hosts: u8, // 4 by default can be changed
}

impl Instance {
    /// Create a new server instance
    pub fn new(port: u32, frequency: u32) -> Self {
        Self { port, frequency,max_hosts:DEFAULT_MAX_HOSTS, }
    }

    /// Set the max number of hosts
    pub fn set_max_hosts(&mut self, value: u8) {
        self.max_hosts = value;
    }

    pub fn run(&self) -> std::io::Result<()> {
        let socket = UdpSocket::bind(format!("{}:{}",Ipv4Addr::new(127, 0, 0, 1),self.port))?;
        socket.set_nonblocking(true)?;
        let instance = self.clone();
        thread::spawn(move || {
            match running(socket, &instance) {
                Ok(_) => println!("server stopped succesfully"),
                Err(e) => println!("SERVER ERROR : {}",e.as_ref())
            };
        });
        Ok(())
    }
}

/// Running loop of the server
fn running(socket: UdpSocket,instance: &Instance) -> Result<(),Error>  {
let mut hosts = Hosts::new();
loop {
    let data = InputData::parse(&socket)?;
    match data {
        InputData::Connection(data) => {
            let addr = data.addr;
            connection(&mut hosts, data, &socket, instance.max_hosts)?;
            println!("{:?}: connection", addr);
        },
        InputData::Update(data) => update(&mut hosts, data, &socket)?,
        InputData::None => (),
        InputData::Unknown => eprintln!("malformed request :\n{:#?}",data),
        _ => eprintln!("not implemented yet"),
    }
}
}