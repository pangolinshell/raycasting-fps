///////////////////////////////////////////////Random Actions//////////////////////////////////////////////////////////////////////
// use tokio::net::UdpSocket;
// use tokio::time::{interval, Duration, timeout};
// use shared::{ClientMessage, ServerMessage, Direction};
// use rand::Rng;

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let server_addr = "127.0.0.1:8080";
//     let name = format!("Bot{}", rand::thread_rng().gen_range(1000..9999));

//     let socket = UdpSocket::bind("0.0.0.0:0").await?;
//     socket.connect(&server_addr).await?;

//     // Send Join message
//     let join_msg = ClientMessage::Join(name.clone());
//     socket.send(&bincode::serialize(&join_msg)?).await?;

//     let mut buf = [0; 1024];

//     // Receive welcome message
//     let len = socket.recv(&mut buf).await?;
//     if let Ok(ServerMessage::Welcome(id)) = bincode::deserialize(&buf[..len]) {
//         println!("✅ {} joined as player ID: {}", name, id);
//     } else {
//         println!("❌ Did not receive welcome message.");
//         return Ok(());
//     }

//     let mut ticker = interval(Duration::from_millis(500));

//     loop {
//         ticker.tick().await;

//         // Randomly decide to move or shoot
//         let action = rand::thread_rng().gen_range(0..2);

//         match action {
//             0 => {
//                 let dir = match rand::thread_rng().gen_range(0..4) {
//                     0 => Direction::Up,
//                     1 => Direction::Down,
//                     2 => Direction::Left,
//                     _ => Direction::Right,
//                 };
//                 let move_msg = ClientMessage::Move(dir);
//                 socket.send(&bincode::serialize(&move_msg)?).await?;
//                 println!("🚶 [{}] Moving in direction: {:?}!", name, dir);
//             }
//             1 => {
//                 let shoot_msg = ClientMessage::Shoot;
//                 socket.send(&bincode::serialize(&shoot_msg)?).await?;
//                 println!("🔫 [{}] Shooting!", name);
//             }
//             _ => {}
            
//         }
        
        
//         // Try to receive server update with timeout
//         match timeout(Duration::from_millis(100), socket.recv(&mut buf)).await {
//             Ok(Ok(len)) => {
//                 if let Ok(ServerMessage::GameState(players)) = bincode::deserialize(&buf[..len]) {
//                     println!("📡 [{}] Players:", name);
//                     for p in players {
//                         println!(
//                             "🧍 {} (ID {}): x = {:.2}, y = {:.2}, movement = {:?}",
//                             p.name, p.id, p.x, p.y, p.movement
//                         );
//                     }
//                 }
//             }
//             _ => {
//                 // Timeout or other error, just continue
//                 println!("⏳ [{}] No game state received", name);
//             }
//         }
//         // match ServerMessage::FullMapState { layout, spawnpoints: (), players: () } =>{
//         //     println!("Map layout: {}", layout);
//         //     println!("Spawnpoints: {}", spawnpoints);
//         //     println!("Players: {}", players);
//         // }
//     }
// }


////////////////////////////////////////////////////////////OUT OF BOUNDS ///////////////////////////////////////////////////////////////////////////

// use tokio::net::UdpSocket;
// use tokio::time::{interval, Duration, timeout};
// use shared::{ClientMessage, ServerMessage, Direction};
// use rand::Rng;

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let server_addr = "127.0.0.1:8080";
//     let name = format!("Bot{}", rand::thread_rng().gen_range(1000..9999));

//     let socket = UdpSocket::bind("0.0.0.0:0").await?;
//     socket.connect(&server_addr).await?;

//     // Send Join message
//     let join_msg = ClientMessage::Join(name.clone());
//     socket.send(&bincode::serialize(&join_msg)?).await?;

//     let mut buf = [0; 1024];

//     // Receive welcome message
//     let len = socket.recv(&mut buf).await?;
//     if let Ok(ServerMessage::Welcome(id)) = bincode::deserialize(&buf[..len]) {
//         println!("✅ {} joined as player ID: {}", name, id);
//     } else {
//         println!("❌ Did not receive welcome message.");
//         return Ok(());
//     }

//     let mut ticker = interval(Duration::from_millis(500));

//     loop {
//         ticker.tick().await;

//         // Always move UP
//         let move_msg = ClientMessage::Move(Direction::Up);
//         socket.send(&bincode::serialize(&move_msg)?).await?;
//         println!("🚶 [{}] Moving UP!", name);

//         // Try to receive server update with timeout
//         match timeout(Duration::from_millis(100), socket.recv(&mut buf)).await {
//             Ok(Ok(len)) => {
//                 if let Ok(ServerMessage::GameState(players)) = bincode::deserialize(&buf[..len]) {
//                     println!("📡 [{}] Players:", name);
//                     for p in players {
//                         println!(
//                             "🧍 {} (ID {}): x = {:.2}, y = {:.2}, movement = {:?}",
//                             p.name, p.id, p.x, p.y, p.movement
//                         );
//                     }
//                 }
//             }
//             _ => {
//                 // Timeout or other error, ignore silently
//             }
//         }
//     }
// }
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////:
use tokio::net::UdpSocket;
use tokio::time::{interval, Duration, timeout};
use shared::{ClientMessage, ServerMessage, Direction};
use rand::Rng;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_addr = "127.0.0.1:8080";
    let name = format!("Bot{}", rand::thread_rng().gen_range(1000..9999));

    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect(&server_addr).await?;

    let join_msg = ClientMessage::Join(name.clone());
    socket.send(&bincode::serialize(&join_msg)?).await?;

    let mut buf = [0; 4096];
    let mut got_welcome = false;
   // let mut got_map = false;

    for _ in 0..2 {
        let len = socket.recv(&mut buf).await?;
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
                   // got_map = true;
                }
                _ => {}
            }
        } else {
            println!("❌ Failed to deserialize message");
        }
    }

    if !got_welcome {
        println!("❌ Did not receive welcome message.");
        return Ok(());
    }

    let mut ticker = interval(Duration::from_millis(500));

    loop {
        ticker.tick().await;

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
                socket.send(&bincode::serialize(&move_msg)?).await?;
                println!("🚶 [{}] Moving: {:?}", name, dir);
            }
            1 => {
                let shoot_msg = ClientMessage::Shoot;
                socket.send(&bincode::serialize(&shoot_msg)?).await?;
                println!("🔫 [{}] Shooting!", name);
            }
            _ => {}
        }

        match timeout(Duration::from_millis(100), socket.recv(&mut buf)).await {
            Ok(Ok(len)) => {
                if let Ok(ServerMessage::GameState(players)) = bincode::deserialize(&buf[..len]) {
                    println!("📡 [{}] Players:", name);
                    for p in players {
                        println!("🧍 {} (ID {}): ({}, {})", p.name, p.id, p.x, p.y);
                    }
                }
            }
            _ => {}
        }
    }
}