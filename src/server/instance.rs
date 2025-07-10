use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::thread;
use crate::server::data::{Deny, Entity, OnConnection};

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
    max_hosts: u8, // default 4,
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
    let mut hosts: Vec<Entity> = Vec::new();
    loop {
            let mut buf = [0;1024];
            let opts = match socket.recv_from(&mut buf) {
                Ok(v) => Some(v),
                // ! IMPORTANT NON BLOQUANT
                Err(e) => {
                    if !matches!(e.kind(), std::io::ErrorKind::WouldBlock) {return Err(Box::new(e));}
                    None
                }
            };
            match opts {
                Some((size,addr)) => {
                    let data = String::from_utf8(buf[..size].to_vec())?;
                    //* MAX PLAYER CASE
                    if hosts.len() == instance.max_hosts as usize { 
                        let data = Deny {reason: format!("max number ({}) of players has been reached ",instance.max_hosts)};
                        socket.send_to(data.to_string()?.as_bytes(), addr)?;
                        continue;
                    }
                    //* FIRST CONNECTION (Handshake)
                    if !hosts.iter().any(|v | v.addr == addr.to_string()) {
                        handshake(data, addr, &mut hosts, &socket)?;                 
                    } 
                    else {

                    }
                }
                None => {},
            }
            // dbg!(&hosts);
        }
}

/// Handle all first connection between the server and client
fn handshake(data: String,addr: SocketAddr, hosts: &mut Vec<Entity>,socket: &UdpSocket) -> Result<(),Error>{
    let v = OnConnection::from_string(data)?;
    let e = Entity::new(addr, v.nickname, (16.0,16.0,0.0));
    hosts.push(e.clone());
    let data = e.to_string()?;
    let addrs = hosts.iter().map(|v| v.clone().addr).collect::<Vec<String>>();
    let _ = broadcast(socket.try_clone().unwrap(), None, addrs, data);
    Ok(())
}

fn broadcast(socket: UdpSocket,from: Option<SocketAddr>,hosts: Vec<String>,data: String) -> std::io::Result<()> {
    for addr in hosts {
        match from {
            Some(v) => if v.to_string() == addr {continue;},
            None => {},
        }
        socket.send_to(data.as_bytes(), addr)?;
    }
    Ok(())
}