#![allow(unused_imports, dead_code)]
use anyhow::Result;
use log;
use crossterm::{cursor, execute, terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    }
};
use std::io::stdout;
use std::path::PathBuf;
use std::sync::{Mutex, Arc};
use crate::command::{read_command, Command, CommandRunner};
use crate::config::Config;
use crate::node_map::NodeMap;
use crate::viewer::{Viewer, ConsoleMessageStatus};


// シングルスレッドなのでRc+RefCellでいいけど，いずれマルチスレッドに拡張したいのでArc+Mutexにしておく
pub struct App {
    node_map: Arc<Mutex<NodeMap>>,
    viewer: Arc<Mutex<Viewer>>,
    command_runner: CommandRunner,
    config: Arc<Config>,
}

impl App {
    pub fn new(root: PathBuf, config: Config) -> Result<App> {
        let config = Arc::new(config);
        let node_map = Arc::new(Mutex::new(NodeMap::new(root, config.clone())?));
        let viewer = Arc::new(Mutex::new(Viewer::new(node_map.clone(), config.clone())));
        let command_runner = CommandRunner::new(node_map.clone(), viewer.clone(), config.clone());
        Ok(App {
            node_map: node_map,
            viewer: viewer,
            command_runner: command_runner,
            config: config,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;

        loop {
            
            let mut viewer = self.viewer.lock().unwrap();
            viewer.sync()?; // viewerとnode_mapの同期
            viewer.display()?; // 表示

            // 標準入力からコマンドを取得 ------------------------
            let command = match read_command() {
                Err(e) => {
                    viewer.set_console_message(format!("{}", e), ConsoleMessageStatus::Error);
                    continue;
                }
                Ok(command) => {
                    if command == Command::Quit{ break }
                    viewer.clear_console_message();
                    command
                }
            };
            log::info!("Command accepted: {:?}", command);

            // command runner内でviewerをborrowするのでこのスコープ内ではドロップ
            std::mem::drop(viewer);

            // コマンドの実行
            let result = self.command_runner.run_command(command);
            if self.config.auto_update {
                self.command_runner.run_command(Command::Update)?;
            }

            // コマンドがエラーだったら表示
            if let Err(e) = result {
                let mut viewer = self.viewer.lock().unwrap();
                viewer.set_console_message(format!("{}", e), ConsoleMessageStatus::Error);
            }
        }

        execute!(stdout(), cursor::Show, LeaveAlternateScreen)?;
        disable_raw_mode()?;
        
        Ok(())
    }
}

