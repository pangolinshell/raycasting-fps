mod args;
use args::Args;
use clap::Parser;

mod logic;

use std::{error::Error, net::{SocketAddr, UdpSocket}, time::Duration};

use crate::logic::connection;

fn main() -> Result<(),Box<dyn Error>> {
    let args = Args::parse();
    let server: SocketAddr = format!("{}:{}",args.host,args.port).parse()?;
    let mut socket = UdpSocket::bind(format!("0.0.0.0:0"))?;
    socket.set_read_timeout(Some(Duration::from_secs(5)))?;
    let (player_data,others_data,map_loader) = connection(&mut socket, server, args.nickname)?;
    Ok(())
}