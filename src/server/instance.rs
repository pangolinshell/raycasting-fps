use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const DEFAULT_MAX_HOSTS: u8 = 4;

pub struct Instance {
    port: u32,
    frequency: u32,

    max_hosts: u8, // default 4,
}

impl Instance {
    pub fn new(port: u32, frequency: u32) -> Self {
        Self { port, frequency,max_hosts:DEFAULT_MAX_HOSTS, }
    }

    pub fn set_max_hosts(&mut self, value: u8) {
        self.max_hosts = value;
    }

    pub fn run(&self) -> std::io::Result<()> {
        let sock = UdpSocket::bind(format!("{}:{}",Ipv4Addr::new(127, 0, 0, 1),self.port))?;
        sock.set_nonblocking(true)?;
        let mut hosts: Vec<SocketAddr> = Vec::new();
        thread::spawn(move || {
            loop {
                let mut buf = [0;1024];
                let opts = match sock.recv_from(&mut buf) {
                    Ok(v) => Some(v),
                    // ! IMPORTANT NON BLOQUANT
                    Err(e) => {
                        if !matches!(e.kind(), std::io::ErrorKind::WouldBlock) {
                            eprintln!("{}",e);
                        }
                        None
                    }
                };
                match opts {
                    Some((size,addr)) => {
                        if !hosts.contains(&addr) {
                        }
                    }
                    None => {},
                }
            }
        });
        Ok(())
    }
}
