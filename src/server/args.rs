pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// launch the multiplayer fps server
pub struct Args {
    /// maximum number of hosts on the session
    #[arg(long="max-hosts",default_value_t=4)]
    pub max_hosts: u8,

    /// port number of the server. is random by default
    #[arg(short,long,default_value_t=0)]
    pub port: u32,

    #[arg(short,long)]
    pub map: String,
}
