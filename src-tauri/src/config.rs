use std::io::{Error, ErrorKind};
use serde::{Deserialize};

const CONFIG_PATH: &str = "../config.toml";

#[derive(Deserialize)]
pub struct AppConfig {
    pub url: String,
    pub exit_shortcut: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, Error> {
        let content = std::fs::read_to_string(CONFIG_PATH)?;
        match toml::from_str(&content) {
            Ok(config) => { Ok(config) }
            Err(err) => { Err(Error::new(ErrorKind::InvalidData, err)) }
        }
    }
}
