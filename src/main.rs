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
use clap::Parser;
use crate::app::App;
use crate::config::load_config;
use crate::utils::path::{get_cwd_path, resolve_path};

#[tokio::main]
async fn main() -> Result<()> {
    
    // 各種初期化
    init_logger(LevelFilter::Info, "app.log")?;
    let config = load_config()?;
    let args = Args::parse();
    
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

    Ok(())
}

fn init_logger(level: LevelFilter, filename: &str) -> Result<()> {
    WriteLogger::init(
        level,
        simplelog::Config::default(),
        File::create(filename).unwrap(),
    )?;

    return Ok(());
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, value_parser)]
    path: Option<PathBuf>,
    #[arg(short, long)]
    sync: bool,
}