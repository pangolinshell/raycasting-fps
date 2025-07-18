use std::net::SocketAddr;
use serde::{Deserialize, Serialize};

use crate::data::{Connection, Status, Update,default_addr};

/// Represents a connected Player with its state and identity.
#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct PlayerData {
    /// IP address and port of the Player.
    #[serde(skip, default = "default_addr")]
    pub addr: SocketAddr,

    /// Player's nickname or identifier.
    pub nickname: String, // Nicknames are uniques

    /// X coordinate of the Player's position.
    pub x: f32,

    /// Y coordinate of the Player's position.
    pub y: f32,

    /// Direction (angle or heading) of the Player.
    pub d: f32,

    /// Current status (Alive, Disconnected, etc.).
    pub status: Status,
}

impl PlayerData {
    /// Initializes a new `Player` from a `Connection` message and initial coordinates.
    ///
    /// # Arguments
    /// * `value` - The incoming `Connection` containing address and nickname.
    /// * `xyd` - A tuple containing initial x, y, and direction values.
    ///
    /// # Returns
    /// A new `Player` instance.
    pub fn init(value: Connection, xyd: (f32, f32, f32)) -> Self {
        Self {
            addr: value.addr,
            nickname: value.nickname,
            x: xyd.0,
            y: xyd.1,
            d: xyd.2,
            status: Status::Alive,
        }
    }

    /// Updates the current Player state with non-None values from an `Update`.
    ///
    /// # Arguments
    /// * `data` - An `Update` struct containing optional new values.
    ///
    /// # Returns
    /// The number of fields that were actually updated (0 to 4).
    pub fn update(&mut self, data: Update) -> u8 {
        let mut modif_datas: u8 = 0;

        if let Some(x) = data.x {
            self.x = x;
            modif_datas += 1;
        }
        if let Some(y) = data.y {
            self.y = y;
            modif_datas += 1;
        }
        if let Some(d) = data.d {
            self.d = d;
            modif_datas += 1;
        }
        if let Some(status) = data.status {
            self.status = status;
            modif_datas += 1;
        }

        modif_datas
    }
}
