use std::{error::Error, net::{SocketAddr, UdpSocket}};

use multiplayer_fps::{data::{Connection, Deny, InputData, OutputData, Status, Update}, entities::{Player, Players}, world::Map};
use multiplayer_fps::Loader;
use rand::prelude::*;

use crate::args::Args;


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
    players: &Players,
    data: String,
) -> std::io::Result<()> {
    if players.len() <= 1 {
        return Ok(());
    }
    for addr in players.iter() {
        match from {
            Some(current_host) => if current_host == addr.addr { continue; },
            None => {},
        }
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
pub fn connection(players: &mut Players,data: Connection,socket: &UdpSocket,max_hosts: u8,loader: Loader) -> Result<(),Box<dyn Error>>{
    if players.get_by_nickname(&data.nickname).is_some() {
        let msg = OutputData::AccessDeny(Deny {reason: format!("the nickname \"{}\" is already used",data.nickname)});
        let serialized = serde_json::to_string(&msg)?;
        socket.send_to(serialized.as_bytes(),data.addr)?;
    }
    if players.get_by_addr(&data.addr).is_some() {
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
    // let new_host = PlayerData::init(data, (16.0,16.0,16.0));
    let mut new_host = Player::new(data.nickname, (16.0,16.0,0.0), "goblin");
    new_host.addr = data.addr;
    let msg = OutputData::New(new_host.clone());
    let serialized = serde_json::to_string(&msg)?;
    // Send new host data to all Players
    let hosts_without_new = players.clone();
    players.push(new_host.clone());
    broadcast(socket, Some(addr), players, serialized)?;

    // Send other Players data to all other users
    let msg = OutputData::Connecting((new_host,hosts_without_new.clone(),loader));
    let serialized = serde_json::to_string(&msg)?;
    socket.send_to(serialized.as_bytes(), addr)?;
    Ok(())
}

// TODO : Add shooting verification
pub fn update(players: &mut Players,data: Update,socket: &UdpSocket) -> Result<(),Box<dyn Error>> {
    players.update(&data);
    let msg = OutputData::Update(data.clone());
    let serialized = serde_json::to_string(&msg)?;
    broadcast(socket, Some(data.addr), players, serialized)?;
    Ok(())
}

pub fn disconnection(players: &mut Players, addr: SocketAddr) -> Result<(),Box<dyn Error>> {
    let index = match players.get_by_addr(&addr) {
        Some(i) => i,
        None => {
            let mut v = vec![];
            for z in players.iter() {
                v.push(z.addr);
            }
            dbg!(v);
            return  Err(format!("the ip {} is not connected",addr).into())
        }
    };
    players.remove(index);
    Ok(())
}

pub fn shoot(players: &mut Players,map: &Map,data: Update,socket: &UdpSocket) -> Result<(),Box<dyn Error>>  {
    const HIT_RADIUS: f32 = 0.5; // ** A magic variable
    const DEATH_TIMOUT: u64 = 0;

    match players.update(&data) {
        Some(v) => v,
        None => return Err(format!("player \"{}\" does not exist", data.nickname).into())
    };
    let p_index = match players.get_by_nickname(&data.nickname) {
        Some(v) => v,
        None => return Err(format!("player \"{}\" does not exist", data.nickname).into())
    };
    let player = match players.get(p_index) {
        Some(p) => p,
        None => return Err(format!("no player \"{}\" on index {}", data.nickname, p_index).into())
    };
    match player.shoot(map, players, HIT_RADIUS) {
        Some(target) => {
            let mut rng = rand::rng();
            let spawn = match map.spawn_points.choose(&mut rng) {
                Some(s) => s,
                None =>  return Err(format!("no spawn point found").into()),
            };
            let data = Update { addr:target.addr, nickname: target.nickname.clone(), x: Some(spawn.pos.x as f32 + 0.5), y: Some(spawn.pos.y as f32 + 0.5), d: Some(target.d), status: Some(Status::Dead(DEATH_TIMOUT)) };
            update(players, data, socket)?;
            println!("{} has been shot",target.nickname);
        }
        None => {}
    }
    update(players, data, socket)?;
    Ok(())
}

pub fn running(socket: UdpSocket,instance: &Args) -> Result<(),Box<dyn Error>>  {
let mut players = Players::new();
let map_loader = Loader::from_file(&instance.map)?;
let map = Map::from(&map_loader);
loop {
    let data = InputData::parse(&socket)?;
    match data {
        InputData::Connection(data) => {
            let addr = data.addr;
            connection(&mut players, data, &socket, instance.max_hosts,map_loader.clone())?;
            println!("{:?}: connection", addr);
        },
        InputData::Update(data) => {
            update(&mut players, data, &socket)?;
        },
        InputData::Disconnection {addr} => {
            disconnection(&mut players, addr)?;
            println!("the player of addr : {} has been succesfully removed",addr)
        }
        InputData::Shoot(data) => {
            shoot(&mut players,&map,data, &socket)?;
        }
        InputData::None => (),
        InputData::Unknown => eprintln!("malformed request :\n{:#?}",data),
    }
}
}