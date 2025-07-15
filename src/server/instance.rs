use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::thread;

use crate::data::{Update,DataType,Connection};

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
let mut hosts: Vec<Update> = Vec::new();
loop {
    let data = DataType::parse(&socket)?;
    match data {
        DataType::Connection(value) => {},
        _ => eprintln!("not implemented yet"),
    }
}
}

/// Handles a new connection ("handshake") from a client.
///
/// # Arguments
/// * `data` - A `String` received from the client, expected to contain connection metadata (e.g., nickname).
/// * `addr` - The `SocketAddr` (IP and port) of the client sending the data.
/// * `hosts` - A mutable reference to the list of known hosts in the network.
/// * `socket` - The `UdpSocket` used to communicate with clients.
///
/// # Returns
/// * `Result<(), Error>` - Returns Ok if the handshake succeeds, or an Error if parsing or serialization fails.
fn handshake(
    data: String,
    addr: SocketAddr,
    hosts: &mut Vec<Update>,
    socket: &UdpSocket,
) -> Result<(), Error> {
    // Parse the received string to extract connection info (e.g., nickname).
    let v = Connection::from_string(data)?;

    // Create a new entity representing this client, with a default position.
    // in xyd : posX, posY, direction in rad
    // TODO: ADD SPAWN CONFIGURATION
    let e = Update::new(addr, v.nickname, (16.0, 16.0, 0.0));
    hosts.push(e.clone());
    let data = e.to_string()?;
    let addrs = hosts.iter().map(|host| host.clone().addr).collect::<Vec<SocketAddr>>();
    let _ = broadcast(socket.try_clone().unwrap(), None, addrs, data);

    Ok(())
}

/// Broadcasts a message to a list of socket addresses via UDP.
///
/// # Arguments
/// * `socket` - The UDP socket to use for sending messages.
/// * `from` - An optional address to exclude from the broadcast (e.g., the sender).
/// * `hosts` - A list of socket addresses to which the message should be sent.
/// * `data` - The message (as a string) to be broadcast.
///
/// # Returns
/// * `std::io::Result<()>` - Returns Ok if all messages are sent successfully, or an error otherwise.
fn broadcast(
    socket: UdpSocket,
    from: Option<SocketAddr>,
    hosts: Vec<SocketAddr>,
    data: String,
) -> std::io::Result<()> {
    for addr in hosts {
        // Skip sending the message back to the sender (if specified).
        match from {
            Some(current_host) => if current_host == addr { continue; },
            None => {},
        }

        // Send the data to the target address.
        socket.send_to(data.as_bytes(), addr)?;
    }

    Ok(())
}
