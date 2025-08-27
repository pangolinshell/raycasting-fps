use std::{ net::{SocketAddr, UdpSocket}, sync::{Arc, atomic::AtomicBool, mpsc::{Receiver, Sender, TryRecvError, channel}}, thread, time::Duration};
use multiplayer_fps::data::{default_addr, Connection, InputData, OutputData};

type Error = Box<dyn std::error::Error>;

#[derive(Debug,Clone)]
pub struct UdpThread {
    atomic: Arc<AtomicBool>,
}

impl UdpThread {
    fn new() -> UdpThread {
        Self { atomic: Arc::new(AtomicBool::new(false)) }
    }

    pub fn kill(&self) {
        self.atomic.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn is_dead(&self) -> bool {
        self.atomic.load(std::sync::atomic::Ordering::Relaxed)
    }
}

pub fn connection(server: SocketAddr,nickname: String,timeout: Option<Duration>) -> Result<(Sender<InputData>, Receiver<OutputData>,UdpThread), Error> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect(server)?;
    socket.set_nonblocking(true)?;
    socket.set_read_timeout(timeout)?;
    let killswitch = UdpThread::new();
    let (input_tx, input_rx) = channel::<InputData>();
    let (output_tx, output_rx) = channel::<OutputData>();

    let socket_clone = socket.try_clone()?;
    let kill_switch_clone = killswitch.clone();
    thread::spawn(move  || {
        if let Err(e) = connection_loop(socket_clone, input_rx, output_tx, nickname,kill_switch_clone) {
            eprintln!("Erreur dans le thread de communication : {e}");
        }
    });

    Ok((input_tx, output_rx,killswitch.clone()))
}

fn connection_loop(
    socket: UdpSocket,
    input_rx: Receiver<InputData>,
    output_tx: Sender<OutputData>,
    nickname: String,
    kill_switch: UdpThread,
) -> Result<(), Error> {
        let data = InputData::Connection(Connection {addr: default_addr(),nickname});
        let serialized = serde_json::to_string(&data)?;
        socket.send(serialized.as_bytes())?;
    loop {
        if kill_switch.is_dead() {
            break;
        }
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
    Ok(())
}
