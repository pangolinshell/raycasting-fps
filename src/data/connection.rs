use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use super::entity::default_addr;

#[derive(Deserialize,Serialize,Debug)]
pub struct Connection {
    #[serde(skip, default = "default_addr")]
    pub addr: SocketAddr,

    pub nickname: String,
}

impl Connection {
    pub fn to_string(&self) -> Result<std::string::String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_string(data: String) -> Result<Self, Box<dyn std::error::Error>> {
        dbg!(&data); // Affiche la chaîne reçue
        let v: Self = serde_json::from_str(&data.as_str())?;
        Ok(v)
    }
}