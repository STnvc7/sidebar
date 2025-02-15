use anyhow::Result;
use dir;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub editor_command: String,
    pub ignore: Vec<String>,
    pub export_variable_name: String,
    pub nerd_font: bool,
    // pub efficient_mode: bool,
    // pub auto_update: bool,
    // pub wrap_line: bool;
}

pub fn load_config() -> Result<Config> {
    // let home_dir = dir::home_dir().unwrap();
    // let config_path = home_dir.join(".sidebar/config.json");

    // テストのときは配下のコンフィグ
    let config_path = "./config.json";

    let config = if Path::new(config_path).exists() {
        let config_json = fs::read_to_string(config_path)?;
        let _config = serde_json::from_str(&config_json)?;
        _config
    }else{
        Config{
            editor_command: String::new(),
            ignore: Vec::new(),
            export_variable_name: String::from("CWD"),
            nerd_font: false,
        }
    };

    return Ok(config);
}
