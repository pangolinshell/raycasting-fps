use std::{ net::{IpAddr, SocketAddr, UdpSocket}, sync::mpsc::{channel, Receiver, Sender, TryRecvError}, thread, time::Duration};
use multiplayer_fps::data::{default_addr, Connection, InputData, OutputData};

type Error = Box<dyn std::error::Error>;

pub fn connection(server: SocketAddr,nickname: String,timeout: Option<Duration>) -> Result<(Sender<InputData>, Receiver<OutputData>), Error> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect(server)?;
    socket.set_nonblocking(true)?;
    socket.set_read_timeout(timeout)?;
    let (input_tx, input_rx) = channel::<InputData>();
    let (output_tx, output_rx) = channel::<OutputData>();

    let socket_clone = socket.try_clone()?;
    thread::spawn(move  || {
        if let Err(e) = connection_loop(socket_clone, input_rx, output_tx, nickname) {
            eprintln!("Erreur dans le thread de communication : {e}");
        }
    });

    Ok((input_tx, output_rx))
}

fn connection_loop(
    socket: UdpSocket,
    input_rx: Receiver<InputData>,
    output_tx: Sender<OutputData>,
    nickname: String
) -> Result<(), Error> {
        let data = InputData::Connection(Connection {addr: default_addr(),nickname});
        let serialized = serde_json::to_string(&data)?;
        socket.send(serialized.as_bytes())?;
    loop {
        match input_rx.try_recv() {
            Ok(v) => {
                let serialized = serde_json::to_string(&v)?;
                socket.send(serialized.as_bytes())?;
            },
            Err(TryRecvError::Empty) => (),
            Err(e) => return Err(Box::new(e)),
        }; // peut renvoyer RecvError

        let mut buf = [0u8; 4096];
        let size = match socket.recv(&mut buf) {
            Ok(s) => s,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
            Err(e) => return Err(Box::new(e)),
        };

        let data = String::from_utf8(buf[..size].to_vec())?;
        let output = serde_json::from_str::<OutputData>(&data).unwrap_or(OutputData::Unknown);
        let _ = output_tx.send(output);
    }
}
