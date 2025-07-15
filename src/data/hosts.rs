use std::net::SocketAddr;
use crate::data::Update;
use std::ops::{Deref, DerefMut};

#[derive(Debug,Clone)]
pub struct Hosts {
    hosts: Vec<Update>
}

impl Hosts {
    pub fn new() -> Self {
        Self { hosts: vec![] }
    }

    pub fn from(v: Vec<Update>) -> Self {
        Self { hosts: v }
    }

    pub fn get_from_addr(&self,addr: SocketAddr) -> Option<&Update> {
        for host in &self.hosts {
            if host.addr == addr {
                return Some(host);
            }
        }
        None
    }

    pub fn get_from_nickname(&self,nickname: &str) -> Option<&Update> {
        for host in &self.hosts {
            if host.nickname == nickname {
                return Some(host);
            }
        }
        None
    }
}

impl Deref for Hosts {
    type Target = Vec<Update>;
    fn deref(&self) -> &Self::Target {
        &self.hosts
    }
}

impl DerefMut for Hosts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.hosts
    }
}