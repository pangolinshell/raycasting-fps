use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Deserialize,Serialize)]
pub struct OnConnection {
    nickname: String,
}

