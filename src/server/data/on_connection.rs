use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct OnConnection {
    pub nickname: String,
}

impl OnConnection {
    pub fn to_string(&self) -> Result<std::string::String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_string(data: String) -> Result<Self, Box<dyn std::error::Error>> {
        dbg!(&data); // Affiche la chaîne reçue
        let v: Self = serde_json::from_str(&data.as_str())?;
        Ok(v)
    }
}