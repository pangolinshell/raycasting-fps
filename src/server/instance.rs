use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::thread;
use crate::server::data::{Deny, Entity, Host, OnConnection};

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
    let mut hosts: Vec<Host> = Vec::new();
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
                    if !hosts.iter().any(|v | v.addr == addr) {
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
    hosts: &mut Vec<Host>,
    socket: &UdpSocket,
) -> Result<(), Error> {
    // Parse the received string to extract connection info (e.g., nickname).
    let v = OnConnection::from_string(data)?;

    // Create a new entity representing this client, with a default position.
    let e = Entity::new(addr, v.nickname, (16.0, 16.0, 0.0));

    // Create a host instance associated with this entity and address.
    let h = Host::new(e.clone(), addr);

    // Add the new host to the list of connected hosts.
    hosts.push(h.clone());

    // Serialize the entity to a string to send to other clients.
    let data = e.to_string()?;

    // Gather addresses of all current hosts to broadcast the new connection.
    let addrs = hosts.iter().map(|host| host.clone().addr).collect::<Vec<SocketAddr>>();

    // Broadcast the new entity to all other clients.
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
