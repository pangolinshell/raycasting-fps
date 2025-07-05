mod server;
use server::Instance;

use std::net::UdpSocket;
use std::time::Duration;
use std::thread;

fn run_client(server_addr: &str) -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?; // Port aléatoire local
    socket.set_read_timeout(Some(Duration::from_secs(2)))?; // Timeout lecture

    let msg = b"Hello server!";
    socket.send_to(msg, server_addr)?;

    let mut buf = [0u8; 1024];
    match socket.recv_from(&mut buf) {
        Ok((n, src)) => {
            let response = String::from_utf8_lossy(&buf[..n]);
            println!("Réponse de {}: {}", src, response);
        }
        Err(e) => {
            println!("Erreur de réception: {}", e);
        }
    }

    Ok(())
}

fn main() {
    let instance = Instance::new(5000, 60);
    let _ = instance.run();
    loop {
        let _ = run_client("127.0.0.1:5000");
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
