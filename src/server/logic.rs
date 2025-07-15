use std::{error::Error, net::{SocketAddr, UdpSocket}};

use crate::data::{Connection, Deny, Host, Hosts, OutputData};

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
pub fn broadcast(
    socket: UdpSocket,
    from: Option<SocketAddr>,
    hosts: &Hosts,
    data: String,
) -> std::io::Result<()> {
    for addr in hosts.iter() {
        // Skip sending the message back to the sender (if specified).
        match from {
            Some(current_host) => if current_host == addr.addr { continue; },
            None => {},
        }

        // Send the data to the target address.
        socket.send_to(data.as_bytes(), addr.addr)?;
    }
    Ok(())
}


pub fn connection(hosts: &mut Hosts,data: Connection,socket: UdpSocket,max_hosts: u8) -> Result<(),Box<dyn Error>>{
    if hosts.get_from_nickname(&data.nickname).is_some() {
        let msg = OutputData::AccessDeny(Deny {reason: format!("the nickname \"{}\" is already used",data.nickname)});
        let serialized = serde_json::to_string(&msg)?;
        socket.send_to(serialized.as_bytes(),data.addr)?;
    }
    if hosts.get_from_addr(data.addr).is_some() {
        let msg = OutputData::AccessDeny(Deny {reason: format!("the address \"{}\" is already used",data.addr)});
        let serialized = serde_json::to_string(&msg)?;
        socket.send_to(serialized.as_bytes(),data.addr)?;
    }
    if hosts.len() == max_hosts as usize {
        let msg = OutputData::AccessDeny(Deny {reason: format!("server full ({}/{})",hosts.len(),max_hosts)});
        let serialized = serde_json::to_string(&msg)?;
        socket.send_to(serialized.as_bytes(),data.addr)?;
    }
    // TODO : add map modularity
    let new_host = Host::init(data, (16.0,16.0,16.0));
    let msg = OutputData::New(new_host.clone());
    let serialized = serde_json::to_string(&msg)?;
    broadcast(socket, None, hosts, serialized)?;
    hosts.push(new_host);
    Ok(())
}