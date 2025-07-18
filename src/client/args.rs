use std::net::Ipv4Addr;
use clap::Parser;


#[derive(Debug,Parser,Clone)]
#[command(version, about, long_about = None)]
/// launch the multiplayer fps client
pub struct Args {
    /// host addr
    #[arg(long)]
    pub host: Ipv4Addr,

    /// host port
    #[arg(long)]
    pub port: u32,

    /// host port
    #[arg(long)]
    pub nickname: String,
}