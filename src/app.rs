#![allow(unused_imports)]

use anyhow::Result;
use log;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Mutex, Arc};

use crate::command::{read_command, Command, CommandRunner};
use crate::config::Config;
use crate::node_map::NodeMap;
use crate::viewer::{Viewer, ConsoleMessageStatus};

#[allow(dead_code)]
pub struct App {
    node_map: Arc<Mutex<NodeMap>>,
    viewer: Arc<Mutex<Viewer>>,
    command_runner: CommandRunner,
    config: Arc<Config>,
}

#[allow(dead_code, unreachable_code)]
impl App {
    pub fn new(root: PathBuf, config: Config) -> App {
        let config = Arc::new(config);
        let node_map = Arc::new(Mutex::new(NodeMap::new(&root)));
        let viewer = Arc::new(Mutex::new(Viewer::new(node_map.clone(), config.clone())));
        let command_runner = CommandRunner::new(node_map.clone(), viewer.clone(), config.clone());
        App {
            node_map: node_map,
            viewer: viewer,
            command_runner: command_runner,
            config: config,
        }
    }

    pub fn run(&mut self) -> Result<()> {
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
            // command runner内でviewerをborrowするのでこのスコープ内ではドロップ
            std::mem::drop(viewer);

            let result = self.command_runner.run_command(command);

            if let Err(e) = result {
                let mut viewer = self.viewer.lock().unwrap();
                viewer.set_console_message(format!("{}", e), ConsoleMessageStatus::Error);
            }
        }
        return Ok(());
    }
}
