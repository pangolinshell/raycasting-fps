use std::{error::Error, net::{SocketAddr, UdpSocket}};

use crate::data::{Player, Players, Connection, OutputData, Deny, Update};

// use data::{Connection, Deny, Host, Players, OutputData, Update};

/// Broadcasts a message to a list of socket addresses via UDP.
///
/// # Arguments
/// * `socket` - The UDP socket to use for sending messages.
/// * `from` - An optional address to exclude from the broadcast (e.g., the sender).
/// * `Players` - A list of socket addresses to which the message should be sent.
/// * `data` - The message (as a string) to be broadcast.
///
/// # Returns
/// * `std::io::Result<()>` - Returns Ok if all messages are sent successfully, or an error otherwise.
pub fn broadcast(
    socket: &UdpSocket,
    from: Option<SocketAddr>,
    Players: &Players,
    data: String,
) -> std::io::Result<()> {
    for addr in Players.iter() {
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

/// Handles a new connection attempt from a client.
///
/// This function performs several validation checks:
/// - If the nickname is already used, it sends a denial message.
/// - If the address is already used, it sends a denial message.
/// - If the server is full (based on `max_hosts`), it sends a denial message.
/// If all checks pass:
/// - A new `Host` is initialized and added to the list.
/// - A broadcast message is sent to all clients with the new host's data.
///
/// # Arguments
/// * `Players` - The current list of connected Players (mutable reference).
/// * `data` - The connection data received from the client.
/// * `socket` - The UDP socket used for communication.
/// * `max_hosts` - The maximum number of allowed Players.
///
/// # Returns
/// * `Ok(())` on success.
/// * `Err(Box<dyn Error>)` if any error occurs during processing (e.g., serialization or socket errors).
pub fn connection(players: &mut Players,data: Connection,socket: &UdpSocket,max_hosts: u8) -> Result<(),Box<dyn Error>>{
    if players.get_from_nickname(&data.nickname).is_some() {
        let msg = OutputData::AccessDeny(Deny {reason: format!("the nickname \"{}\" is already used",data.nickname)});
        let serialized = serde_json::to_string(&msg)?;
        socket.send_to(serialized.as_bytes(),data.addr)?;
    }
    if players.get_from_addr(data.addr).is_some() {
        let msg = OutputData::AccessDeny(Deny {reason: format!("the address \"{}\" is already used",data.addr)});
        let serialized = serde_json::to_string(&msg)?;
        socket.send_to(serialized.as_bytes(),data.addr)?;
    }
    if players.len() == max_hosts as usize {
        let msg = OutputData::AccessDeny(Deny {reason: format!("server full ({}/{})",players.len(),max_hosts)});
        let serialized = serde_json::to_string(&msg)?;
        socket.send_to(serialized.as_bytes(),data.addr)?;
    }
    // TODO : add map modularity
    let addr = data.addr;
    let new_host = Player::init(data, (16.0,16.0,16.0));
    let msg = OutputData::New(new_host.clone());
    let serialized = serde_json::to_string(&msg)?;
    // Send new host data to all Players
    let hosts_without_new = players.clone();
    players.push(new_host.clone());
    broadcast(socket, Some(addr), players, serialized)?;

    // Send other Players data to all other users
    let msg = OutputData::Connecting((new_host,hosts_without_new.clone()));
    let serialized = serde_json::to_string(&msg)?;
    socket.send_to(serialized.as_bytes(), addr)?;
    Ok(())
}

// TODO : Add shooting verification
pub fn update(Players: &mut Players,data: Update,socket: &UdpSocket) -> Result<(),Box<dyn Error>> {
    let msg = OutputData::Update(data.clone());
    let serialized = serde_json::to_string(&msg)?;
    broadcast(socket, Some(data.addr), Players, serialized)?;
    Ok(())
}