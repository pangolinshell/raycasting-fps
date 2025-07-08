//////////////////////////////////////////////////////////////Client Random Tests//////////////////////////////////////////////////////////////:
use std::net::UdpSocket;
use std::time::{Duration, Instant};
use std::thread;
use rand::Rng;
use shared::{ClientMessage, ServerMessage, Direction};
use bincode;

fn main() -> anyhow::Result<()> {
    let server_addr = "127.0.0.1:8080";
    let name = format!("Bot{}", rand::thread_rng().gen_range(1000..9999));

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect(server_addr)?;
    socket.set_read_timeout(Some(Duration::from_millis(500)))?;

    // Send join message
    let join_msg = ClientMessage::Join(name.clone());
    socket.send(&bincode::serialize(&join_msg)?)?;

    let mut buf = [0; 4096];
    let mut got_welcome = false;

    for _ in 0..2 {
        match socket.recv(&mut buf) {
            Ok(len) => {
                if let Ok(server_msg) = bincode::deserialize::<ServerMessage>(&buf[..len]) {
                    match server_msg {
                        ServerMessage::Welcome(id) => {
                            println!("✅ {} joined as player ID: {}", name, id);
                            got_welcome = true;
                        }
                        ServerMessage::FullMapState { layout, spawnpoints, players } => {
                            println!("🗺️ Map layout:");
                            for row in layout {
                                println!("{:?}", row);
                            }
                            println!("📍 Spawnpoints:");
                            for sp in spawnpoints {
                                println!("({}, {})", sp.x, sp.y);
                            }
                            println!("🧍 Players:");
                            for p in players {
                                println!("- {} at ({}, {})", p.name, p.x, p.y);
                            }
                        }
                        _ => {}
                    }
                } else {
                    println!("❌ Failed to deserialize message");
                }
            }
            Err(e) => {
                println!("⚠️ Receive error: {}", e);
            }
        }
    }

    if !got_welcome {
        println!("❌ Did not receive welcome message.");
        return Ok(());
    }

    // Game loop
    loop {
        let start = Instant::now();

        let action = rand::thread_rng().gen_range(0..2);
        match action {
            0 => {
                let dir = match rand::thread_rng().gen_range(0..4) {
                    0 => Direction::Up,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    _ => Direction::Right,
                };
                let move_msg = ClientMessage::Move(dir);
                socket.send(&bincode::serialize(&move_msg)?)?;
                println!("🚶 [{}] Moving: {:?}", name, dir);
            }
            1 => {
                let shoot_msg = ClientMessage::Shoot;
                socket.send(&bincode::serialize(&shoot_msg)?)?;
                println!("🔫 [{}] Shooting!", name);
            }
            _ => {}
        }

        match socket.recv(&mut buf) {
            Ok(len) => {
                if let Ok(ServerMessage::GameState(players)) = bincode::deserialize(&buf[..len]) {
                    println!("📡 [{}] Players:", name);
                    for p in players {
                        println!("🧍 {} (ID {}): ({}, {})", p.name, p.id, p.x, p.y);
                    }
                }
            }
            Err(_) => {
                println!("⏳ [{}] No game state received", name);
            }
        }

        let elapsed = start.elapsed();
        if elapsed < Duration::from_millis(500) {
            thread::sleep(Duration::from_millis(500) - elapsed);
        }
    }
}