use serde::Deserialize;
use std::{fs, path::Path};
use toml;

#[derive(Deserialize, Debug)]
pub struct Bot {
    pub token: String,
    pub bot_id: u64,
}

#[derive(Deserialize, Debug)]
pub struct Privacy {
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub bot: Bot,
    pub privacy: Privacy,
}

impl Config {
    pub fn new() -> Self {
        if !Path::new("config.toml").exists() {
            panic!("Config file doesnt exist. Xi jinping is watching your File System!");
        }
        let buffer = fs::read_to_string("config.toml").unwrap();
        let config: Self = toml::from_str(&buffer).unwrap();
        config
    }
}
