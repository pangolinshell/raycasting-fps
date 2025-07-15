use std::net::SocketAddr;
use crate::data::{Host, Update};
use std::ops::{Deref, DerefMut};

/// A collection managing multiple `Host` instances.
#[derive(Debug, Clone)]
pub struct Hosts {
    hosts: Vec<Host>,
}

impl Hosts {
    /// Creates a new empty `Hosts` collection.
    pub fn new() -> Self {
        Self { hosts: vec![] }
    }

    /// Creates a `Hosts` collection from an existing vector of `Host`.
    ///
    /// # Arguments
    /// * `v` - Vector of `Host` instances to initialize with.
    pub fn from(v: Vec<Host>) -> Self {
        Self { hosts: v }
    }

    /// Finds a reference to a `Host` by its socket address.
    ///
    /// # Arguments
    /// * `addr` - The socket address to look for.
    ///
    /// # Returns
    /// An option containing a reference to the matching `Host`, or `None` if not found.
    pub fn get_from_addr(&self, addr: SocketAddr) -> Option<&Host> {
        for host in &self.hosts {
            if host.addr == addr {
                return Some(host);
            }
        }
        None
    }

    /// Finds a mutable reference to a `Host` by its socket address.
    ///
    /// # Arguments
    /// * `addr` - The socket address to look for.
    ///
    /// # Returns
    /// An option containing a mutable reference to the matching `Host`, or `None` if not found.
    pub fn get_from_addr_mut(&mut self, addr: SocketAddr) -> Option<&mut Host> {
        for host in &mut self.hosts {
            if host.addr == addr {
                return Some(host);
            }
        }
        None
    }

    /// Finds a mutable reference to a `Host` by its socket address.
    ///
    /// # Arguments
    /// * `addr` - The socket address to look for.
    ///
    /// # Returns
    /// An option containing a mutable reference to the matching `Host`, or `None` if not found.
    pub fn get_from_nickname(&mut self, nickname: &str) -> Option<&Host> {
        for host in &mut self.hosts {
            if host.nickname == nickname.to_string() {
                return Some(host);
            }
        }
        None
    }

    /// Updates a `Host` matching the address contained in the given `Update`.
    ///
    /// # Arguments
    /// * `data` - The update data containing the address and optional fields to update.
    ///
    /// # Returns
    /// - The number of fields updated (as `i8`) if a matching host was found.
    /// - `None` if no host with the given address exists.
    pub fn update(&mut self, data: Update) -> Option<usize> {
        let host = match self.get_from_addr_mut(data.addr) {
            Some(v) => v,
            None => return None,
        };
        Some(host.update(data) as usize)
    }
}

impl Deref for Hosts {
    type Target = Vec<Host>;
    fn deref(&self) -> &Self::Target {
        &self.hosts
    }
}

impl DerefMut for Hosts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.hosts
    }
}