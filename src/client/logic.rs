use std::{net::{SocketAddr, UdpSocket}, sync::mpsc::{Receiver, Sender, TryRecvError}, time::{Duration, Instant}};

use multiplayer_fps::{camera::Camera, data::{default_addr, InputData, OutputData, Update}, entities::{Player, Players}, Loader};

type Error = Box<dyn std::error::Error>;

pub fn on_connection(data: &Receiver<OutputData>) -> Result<(Player, Players, Loader), Error> {
    let timeout = Duration::from_secs(20);
    let start = Instant::now();

    while start.elapsed() < timeout {
        match data.try_recv() {
            Ok(OutputData::Connecting(v)) => return Ok(v),
            Ok(OutputData::None) => {
                return Err("No response received from server.".into());
            }
            Ok(_) => {
                // On ignore les autres messages inattendus
                continue;
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                // Pas de message pour l'instant, on attend un peu
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                return Err("Channel disconnected.".into());
            }
        }
    }

    Err("Timeout: no valid response received.".into())
}


#[allow(unused)]
pub fn disconnection(socket: &mut UdpSocket,server: SocketAddr) -> Result<(),Error> {
    let data = InputData::Disconnection { addr: default_addr() };
    let serialized = serde_json::to_string(&data)?;
    socket.send_to(serialized.as_bytes(), server)?;
    Ok(())
}

pub fn shoot(tx: &Sender<InputData>,camera: Camera,nickname: &str) -> Result<(),Error> {
    let data = InputData::Shoot(Update::new(default_addr(), nickname.to_string(), camera.xyd()));
    tx.send(data)?;
    Ok(())
}

pub fn update(tx: &Sender<InputData>,rx: &Receiver<OutputData>,camera: Camera,nickname: &str,others: &mut Players) -> Result<(),Error> {
    let data = InputData::Update(Update::new(default_addr(), nickname.to_string(), camera.xyd()));
    tx.send(data)?;
    let output = match rcv(rx)? {
        Some(v) => v,
        None => return Ok(()),
    };
    match output {
        OutputData::Update(data) => {others.update(data);},
        OutputData::New(data) => others.push(data),
        _ => (),
    }
    Ok(())
}

pub fn rcv<T>(rx: &Receiver<T>) -> Result<Option<T>,Error> {
    let v = match rx.try_recv() {
        Ok(v) => Some(v),
        Err(e) => {
            if e == TryRecvError::Empty {
                None
            } else {
                return Err(Box::new(e));
            }
        }
    };
    return Ok(v);
}