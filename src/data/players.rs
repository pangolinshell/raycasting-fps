use std::net::SocketAddr;
use serde::{Deserialize, Serialize};

use crate::data::{PlayerData, Update};
use std::ops::{Deref, DerefMut};

/// A collection managing multiple `Player` instances.
#[derive(Serialize,Deserialize,Debug, Clone)]
pub struct PlayersData {
    pub players: Vec<PlayerData>,
}

impl PlayersData {
    /// Creates a new empty `Players` collection.
    pub fn new() -> Self {
        Self { players: vec![] }
    }

    /// Creates a `Players` collection from an existing vector of `Player`.
    ///
    /// # Arguments
    /// * `v` - Vector of `Player` instances to initialize with.
    pub fn from(v: Vec<PlayerData>) -> Self {
        Self { players: v }
    }

    /// Finds a reference to a `Player` by its socket address.
    ///
    /// # Arguments
    /// * `addr` - The socket address to look for.
    ///
    /// # Returns
    /// An option containing a reference to the matching `Player`, or `None` if not found.
    pub fn get_from_addr(&self, addr: SocketAddr) -> Option<&PlayerData> {
        for player in &self.players {
            if player.addr == addr {
                return Some(player);
            }
        }
        None
    }

    /// Finds a mutable reference to a `Player` by its socket address.
    ///
    /// # Arguments
    /// * `addr` - The socket address to look for.
    ///
    /// # Returns
    /// An option containing a mutable reference to the matching `Player`, or `None` if not found.
    pub fn get_from_addr_mut(&mut self, addr: SocketAddr) -> Option<&mut PlayerData> {
        for player in &mut self.players {
            if player.addr == addr {
                return Some(player);
            }
        }
        None
    }

    /// Finds a mutable reference to a `Player` by its socket address.
    ///
    /// # Arguments
    /// * `addr` - The socket address to look for.
    ///
    /// # Returns
    /// An option containing a mutable reference to the matching `Player`, or `None` if not found.
    pub fn get_from_nickname(&mut self, nickname: &str) -> Option<&PlayerData> {
        for player in &mut self.players {
            if player.nickname == nickname.to_string() {
                return Some(player);
            }
        }
        None
    }

    /// Updates a `Player` matching the address contained in the given `Update`.
    ///
    /// # Arguments
    /// * `data` - The update data containing the address and optional fields to update.
    ///
    /// # Returns
    /// - The number of fields updated (as `i8`) if a matching Player was found.
    /// - `None` if no Player with the given address exists.
    pub fn update(&mut self, data: Update) -> Option<usize> {
        let player = match self.get_from_addr_mut(data.addr) {
            Some(v) => v,
            None => return None,
        };
        Some(player.update(data) as usize)
    }
}

impl Deref for PlayersData {
    type Target = Vec<PlayerData>;
    fn deref(&self) -> &Self::Target {
        &self.players
    }
}

impl DerefMut for PlayersData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.players
    }
}