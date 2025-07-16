use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize,Debug,Clone)]
/// Used in case of access denial
pub struct Deny {
    pub reason: String
}

impl Deny {
    pub fn from_string(data: String) -> Result<Self, Box<dyn std::error::Error>> {
        let v: Self = serde_json::from_str(&data.as_str())?;
        Ok(v)
    }

    pub fn to_string(&self) -> Result<std::string::String, serde_json::Error> {
        serde_json::to_string(self)
    }
}