mod app;
mod color;
mod command;
mod config;
mod icon;
mod node;
mod node_map;
mod utils;
mod viewer;

use anyhow::Result;
use std::fs::File;
use std::path::PathBuf;
use simplelog::{LevelFilter, WriteLogger};
use log;
use clap::Parser;
use crate::app::App;
use crate::config::load_config;
use crate::utils::path::{get_cwd_path, resolve_path, get_application_root};

#[tokio::main]
async fn main() -> Result<()> {
    // 各種初期化
    init_logger(LevelFilter::Info)?;
    log::info!("Application start!");
    let config = load_config()?;
    log::info!("Config: {:?}", config);
    let args = Args::parse();
    log::info!("Arguments: {:?}", args);
    
    // if args.sync {
    //     let sync_client = Sync::new();
    //     sync_client.run()?;
    //     return Ok(())
    // }
    
    let path = match args.path {
        Some(p) => resolve_path(p)?,
        None => get_cwd_path()? 
    };
    let mut app = App::new(path, config)?;
    app.run()?;
    log::info!("Close application!");
    Ok(())
}

fn init_logger(level: LevelFilter) -> Result<()> {
    let root = get_application_root()?;
    let log_path = root.join("app.log");
    WriteLogger::init(
        level,
        simplelog::Config::default(),
        File::create(log_path).unwrap(),
    )?;

    return Ok(());
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long, value_parser, help="Specify root path of sidebar")]
    path: Option<PathBuf>,
    #[arg(short, long, help="Synchronize current working directory with another sidebar")]
    sync: bool,
}