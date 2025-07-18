use std::{error::Error, net::{Ipv4Addr, UdpSocket}};
use multiplayer_fps::server::*;
use clap::Parser;
mod args;

fn main() -> Result<(),Box<dyn Error>> {
    let args = args::Args::parse();
    let mut instance = Instance::new(args.port, 100);
    instance.set_max_hosts(args.max_hosts);
    let socket = UdpSocket::bind(format!("{}:{}",Ipv4Addr::new(0, 0, 0, 0),args.port))?;
    println!("running server on port {}",socket.local_addr()?.port());
    running(socket, &instance)?;
    Ok(())
}