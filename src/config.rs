#![allow(unused_imports)]
use anyhow::Result;
use dir;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};
use crate::utils::path::get_application_root;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub sync_server: bool,
    pub editor_command: String,
    pub ignore: Vec<String>,
    pub nerd_font: bool,
    pub skip_exist: bool,
    pub saving_memory: bool,
    pub auto_update: bool,
}

pub fn load_config() -> Result<Config> {
    
    let config_path = if cfg!(debug_assertions) {
        PathBuf::from("./config.yaml")
    } else {
        let root = get_application_root()?;
        root.join("config.yaml")
    };

    if config_path.exists() {
        let config_yaml = fs::read_to_string(config_path)?;
        let config = serde_yaml::from_str(&config_yaml)?;
        return Ok(config)
    } else{
        let config = Config{
            sync_server: true,
            editor_command: String::new(),
            ignore: Vec::new(),
            nerd_font: false,
            skip_exist: true,
            saving_memory: true,
            auto_update: true,
        };

        // 保存
        let serialized = serde_yaml::to_string(&config)?;
        let mut file = fs::File::create(config_path)?;
        write!(file, "{}", serialized)?;

        return Ok(config)
    };
}
