mod app;
mod color;
mod command;
mod config;
mod icon;
mod node;
mod node_map;
mod shell_command;
mod utils;
mod viewer;

use anyhow::{anyhow, Result};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{cursor, execute};
use std::env;
use std::fs::File;
use std::io::stdout;
use std::path::PathBuf;
use {
    simplelog,
    simplelog::{LevelFilter, WriteLogger},
};

use crate::app::App;
use crate::config::load_config;
use crate::utils::path::{get_cwd_path, resolve_path};

#[tokio::main]
async fn main() -> Result<()> {
    init_logger(LevelFilter::Info, "app.log")?;

    let path = parse_arg_and_get_root()?;
    let config = load_config()?;
    let mut app = App::new(path, config);

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;
    app.run()?;
    execute!(stdout(), cursor::Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn parse_arg_and_get_root() -> Result<PathBuf> {
    let mut args = env::args();
    let path = match args.nth(1) {
        Some(_path) => resolve_path(PathBuf::from(_path))?,
        None => get_cwd_path()?,
    };
    if path.is_dir() == false {
        return Err(anyhow!("{:?} does not exist", path));
    }
    return Ok(path);
}

fn init_logger(level: LevelFilter, filename: &str) -> Result<()> {
    WriteLogger::init(
        level,
        simplelog::Config::default(),
        File::create(filename).unwrap(),
    )?;

    return Ok(());
}
