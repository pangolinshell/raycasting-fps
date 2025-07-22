use std::{error::Error, net::{Ipv4Addr, UdpSocket}, time::Duration};
use clap::Parser;
pub mod args;
pub mod logic;


fn main() -> Result<(),Box<dyn Error>> {
    let args = args::Args::parse();
    let socket = UdpSocket::bind(format!("{}:{}",Ipv4Addr::new(0, 0, 0, 0),args.port))?;
    socket.set_read_timeout(Some(Duration::from_secs(5)))?;
    println!("running server on port {}",socket.local_addr()?.port());
    logic::running(socket, &args)?;
    Ok(())
}