mod gamestate;
mod utils;
mod shot;

use std::{ sync::Arc}; //net::SocketAddr
use tokio::{fs::read_to_string, net::UdpSocket, sync::Mutex, time::interval};
use shared::{ServerMessage}; //ClientMessage,
use shared::map::Map;
use gamestate::GameState;
//use serde_json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🔫 FPS UDP Server starting on 127.0.0.1:8080");

   let json_map = read_to_string("../assets/map.jsonc").await
   .expect("❌ Failed to read ../assets/map.jsonc — does the file exist?");
    println!("🗺️ Loaded map: {}", json_map);
    let map: Map = json5::from_str(&json_map)?;
    let map = Arc::new(map);

    let state = Arc::new(Mutex::new(GameState::new(map.clone())));
    let socket = Arc::new(UdpSocket::bind("127.0.0.1:8080").await?);

    // handlers loop
    {
        let socket = socket.clone();
        let state = state.clone();
        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            loop {
                if let Ok((len, addr)) = socket.recv_from(&mut buf).await {
                    if let Ok(msg) = bincode::deserialize(&buf[..len]) {
                        let mut state = state.lock().await;
                        if let Err(e) = gamestate::handle_client_message(&mut state, msg, addr, &socket).await {
                            eprintln!("Error: {}", e);
                        }
                    }
                }
            }
        });
    }

    // Game loop
    {
        let socket = socket.clone();
        let state = state.clone();
        tokio::spawn(async move {
            let mut ticker = interval(std::time::Duration::from_millis(100));
            loop {
                ticker.tick().await;
                let mut state = state.lock().await;
                state.update_positions();
                state.cleanup_inactive_players();

                let players = state.get_game_state();
                let msg = bincode::serialize(&ServerMessage::GameState(players)).unwrap();

                for (addr, _) in &state.addr_to_id {
                    let _ = socket.send_to(&msg, addr).await;
                }
            }
        });
    }
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
