mod gamestate;
mod utils;
mod shot;

use std::{
    fs,
    net::UdpSocket,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use shared::{ServerMessage};
use shared::map::Map;
use gamestate::GameState;
use bincode;
use std::io::ErrorKind;

fn main() -> anyhow::Result<()> {
    println!("🔫 FPS UDP Server starting on 127.0.0.1:8080");

    // Load JSON map from file
    let json_map = fs::read_to_string("../assets/map.jsonc")
        .expect("❌ Failed to read ../assets/map.jsonc — does the file exist?");
    println!("🗺️ Loaded map: {}", json_map);

    let map: Map = json5::from_str(&json_map)?;
    let map = Arc::new(map);
    let state = Arc::new(Mutex::new(GameState::new(map.clone())));

    let socket = Arc::new(UdpSocket::bind("127.0.0.1:8080")?);
    socket.set_nonblocking(true)?;

    // Clone for threads
    let socket_recv = Arc::clone(&socket);
    let socket_game = Arc::clone(&socket);
    let state_recv = Arc::clone(&state);
    let state_game = Arc::clone(&state);

    // ==== Thread 1: Receiving client messages ====
    thread::spawn(move || {
        let mut buf = [0u8; 1024];

        loop {
            match socket_recv.recv_from(&mut buf) {
                Ok((len, addr)) => {
                    if let Ok(msg) = bincode::deserialize(&buf[..len]) {
                        let mut state = state_recv.lock().unwrap();
                        if let Err(error1) = gamestate::handle_client_message(&mut state, msg, addr, &socket_recv) {
                            eprintln!("❌ Error handling client msg: {}", error1);
                        }
                    } else {
                        eprintln!("❌ Failed to deserialize client message");
                    }
                }
            
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
            
                Err(e) => {
                    eprintln!("❌ Socket recv_from error: {}", e);
                }
            }
        }
    });

    thread::spawn(move || {
        loop {
            let tick_start = Instant::now();

            {
                let mut state = state_game.lock().unwrap();
                state.update_positions();
                state.cleanup_inactive_players();

                let players = state.get_game_state();
                let msg = bincode::serialize(&ServerMessage::GameState(players)).unwrap();

                for (addr, _) in &state.addr_to_id {
                    let _ = socket_game.send_to(&msg, addr);
                }
            }

            let elapsed = tick_start.elapsed();
            if elapsed < Duration::from_millis(100) {
                thread::sleep(Duration::from_millis(100) - elapsed);
            }
        }
    });

    // Keep main thread alive
    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}