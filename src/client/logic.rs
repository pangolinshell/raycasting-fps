use std::{net::{SocketAddr, UdpSocket}, sync::mpsc::{Receiver, Sender, TryRecvError}, time::{Duration, Instant}};

use multiplayer_fps::{camera::Camera, data::{self, default_addr, InputData, OutputData, Update}, entities::{Player, Players}, Loader};

type Error = Box<dyn std::error::Error>;
// pub fn connection(socket: &mut UdpSocket,server: SocketAddr,nickname: String) -> Result<(Player,Players,Loader), Error> {
//     let data = InputData::Connection(data::Connection {addr: default_addr(),nickname});
//     let serialized = serde_json::to_string(&data)?;
//     socket.send_to(serialized.as_bytes(), server)?;
//     socket.set_read_timeout(Some(Duration::from_secs(10)))?;
//     let data = OutputData::parse(socket)?;
//     let (player,others,map) = match data {
//         OutputData::Connecting(v) => v,
//         OutputData::None => return Err("No response received from server within the timeout.".into()),
//         _ =>return Err(format!("Unexpected response type: {:?}", data).into()),
//     };
//     Ok((player,others,map))
// }

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



pub fn disconnection(socket: &mut UdpSocket,server: SocketAddr) -> Result<(),Error> {
    let data = InputData::Disconnection { addr: default_addr() };
    let serialized = serde_json::to_string(&data)?;
    socket.send_to(serialized.as_bytes(), server)?;
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