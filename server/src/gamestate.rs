use core::time;
//use std::collections::btree_map::Range;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use shared::{Direction, PlayerState, ClientMessage};
use shared::map::Map;
use tokio::net::UdpSocket;
use crate::{shot}; //gamestate
use shot::ShotResult;
//use net::MapSnapShot;
use tokio::time::{ Duration};
use shared::ServerMessage;

pub struct GameState {
    pub players: HashMap<u32, PlayerState>,
    pub addr_to_id: HashMap<SocketAddr, u32>,
    pub next_id: u32,
    pub map: Arc<Map>,
    pub spawn_index: usize,
    //pub last_tick_events: Vec<GameEvent>,
}

impl GameState {
    pub fn new(map: Arc<Map>) -> Self  {
        Self {
            players: HashMap::new(),
            addr_to_id: HashMap::new(),
            next_id: 1,
            map,
            spawn_index: 0,
       //     last_tick_events: Vec::new(),
        }
    }

    pub fn get_next_spawn(&mut self) -> (i32, i32) {
        let sp = &self.map.spawnpoints[self.spawn_index];
        self.spawn_index = (self.spawn_index + 1) % self.map.spawnpoints.len();
        println!(" Spawn available at ({}, {})",sp.x, sp.y);
        (sp.x as i32, sp.y as i32)
    }

    pub fn update_positions(&mut self) {

        let players = &mut self.players;
        for player in players.values_mut() {
            let (mut x, mut y) = (player.x, player.y);
            match player.movement {
                Some(Direction::Up) => y -= 1,
                Some(Direction::Down) => y += 1,
                Some(Direction::Left) => x -= 1,
                Some(Direction::Right) => x += 1,
                _ => (),
            }
            if self.map.is_walkable(x, y) {
                player.x = x;
                player.y = y;
            }

            player.movement = None;
            player.alive = true;
        }
    }

    pub fn cleanup_inactive_players(&mut self) {
        self.addr_to_id.retain(|_, &mut id| {
            self.players.get(&id).map_or(false, |p| p.alive|| p.inactive )
        });
        self.players.retain(|id, _| self.addr_to_id.values().any(|&v| v == *id));
    }

    pub fn get_game_state(&self) -> Vec<PlayerState> {
        self.players.values().cloned().collect()
    }
    // pub fn snapshot_map_data(&self) -> MapSnapShot {
    //     MapSnapShot {
    //         layout: self.map.layout.clone(),
    //         spawnpoints: self.map.spawnpoints.iter().map(|sp| (sp.x, sp.y)).collect(),
    //         players: self.players.values().map(|p| (p.id, p.x, p.y)).collect(),
    //     }
    // }
}

#[warn(unused_variables)]
pub async fn handle_client_message(
    state: &mut GameState,
    msg: ClientMessage,
    addr: SocketAddr,
    socket: &UdpSocket,
) -> anyhow::Result<()> {
    match msg {
        ClientMessage::Join(name) => {
            let (x, y) = state.get_next_spawn();
            let player = PlayerState {
                id: state.next_id,
                name,
                x,
                y,
                movement: None,
                angle_degrees: 0.0,
                alive: true,
                inactive: false,
                respawning: (false, Duration::from_secs(0)),
            };
        
            state.addr_to_id.insert(addr, player.id);
            state.players.insert(player.id, player.clone());
            state.next_id += 1;
        
            let welcome = ServerMessage::Welcome(player.id);
            let encoded = bincode::serialize(&welcome)?;
            socket.send_to(&encoded, addr).await?;
        
            let full_map = ServerMessage::FullMapState {
                layout: state.map.layout.clone(),
                spawnpoints: state.map.spawnpoints.clone(),
                players: state.get_game_state(),
            };
            let encoded = bincode::serialize(&full_map)?;
            socket.send_to(&encoded, addr).await?;
        }
        ClientMessage::Move(dir) => {
            if let Some(id) = state.addr_to_id.get(&addr) {
                if let Some(player) = state.players.get_mut(id) {
                    let old_x = player.x;
                    let old_y = player.y;
                    player.movement = Some(dir);
                    player.alive = true;
                    match dir {
                        Direction::Up => player.y += 1,
                        Direction::Down => player.y -= 1,
                        Direction::Left => player.x -= 1,
                        Direction::Right => player.x += 1,
                        Direction::None => {}
                    }
                    if player.movement != Option::None && state.map.is_walkable(player.x, player.y) {
                        println!(
                            "🚗 Player {} moved {:?} from ({:.0}, {:.0}) to ({:.0}, {:.0})",
                            player.id, player.movement, old_x, old_y, player.x, player.y
                        );
                    } else {
                        println!("Out of bounds!!! Player staying at ({:.0}, {:.0})", player.x, player.y);
                        player.x = old_x;
                        player.y = old_y;
                    }
                    player.movement = Option::None;
                }
            }
        }
        ClientMessage::Shoot => {
            if let Some(id) = state.addr_to_id.get(&addr) {
                if let Some(player) = state.players.get(id) {
                    let result: ShotResult = GameState::trace_shot_angle(&state ,&player, &state.map, 1000.0);
                    println!(
                        "🔫 Player {} shot from {:?} to {:?}",
                        id,result.init_pos, result.final_pos
                    );
                    if let Some(hit_pos) = result.wall_hit {
                        println!("🧱 Hit wall at {:?}", hit_pos);
                    }
                    if let Some((target_id, hit_pos)) = result.player_hit {
                        if let Some(target) = state.players.get_mut(&target_id) {
                            println!("🎯 Hit player {} at {:?}", target_id, hit_pos);
                            target.alive = false;
                            println!("💀 Player {} died at {:?}", target_id, hit_pos);
                            target.respawning = (true, time::Duration::from_secs(3));
                        }
                    }
                }
            }
        }
        ClientMessage::Respawn => {
            if let Some(id) = state.addr_to_id.get(&addr) {
                if let Some(player) = state.players.get_mut(id) {
                    player.alive = false;
                    player.respawning = (true, time::Duration::from_secs(3));
                    println!("💀 Player {} respawning", id);
                }
            }
        }
        ClientMessage::Disconnect => {
            if let Some(id) = state.addr_to_id.remove(&addr) {
                state.players.remove(&id);
                println!("🔌 Player {} disconnected", id);
            }
        }
    }
    Ok(())
}
