use std::net::SocketAddr;
use crate::data::Host;
use std::ops::{Deref, DerefMut};

#[derive(Debug,Clone)]
pub struct Hosts {
    hosts: Vec<Host>
}

impl Hosts {
    pub fn new() -> Self {
        Self { hosts: vec![] }
    }

    pub fn from(v: Vec<Host>) -> Self {
        Self { hosts: v }
    }

    pub fn get_from_addr(&self,addr: SocketAddr) -> Option<&Host> {
        for host in &self.hosts {
            if host.addr == addr {
                return Some(host);
            }
        }
        None
    }

    pub fn get_from_nickname(&self,nickname: &str) -> Option<&Host> {
        for host in &self.hosts {
            if host.nickname == nickname {
                return Some(host);
            }
        }
        None
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