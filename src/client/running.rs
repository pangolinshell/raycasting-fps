use std::net::UdpSocket;
use std::time::Duration;

use crate::server::data::OnConnection;

pub fn run_client(server_addr: &str,port: u32,name: String) -> std::io::Result<()> {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}",port))?; // Port aléatoire local
    socket.set_read_timeout(Some(Duration::from_secs(2)))?; // Timeout lecture

    let connection_data = OnConnection {nickname: name}.to_string()?;

    let msg = connection_data.as_bytes();
    socket.send_to(msg, server_addr)?;

    let mut buf = [0u8; 1024];
    match socket.recv_from(&mut buf) {
        Ok((n, src)) => {
            let response = String::from_utf8_lossy(&buf[..n]);
            println!("Réponse de {}: {}", src, response);
        }
        Err(e) => {
            if !matches!(e.kind(), std::io::ErrorKind::WouldBlock) {
                println!("Erreur de réception: {}", e);
            }
        }
    }

    Ok(())
}