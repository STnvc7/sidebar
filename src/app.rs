use anyhow::Result;
use log;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::command::{read_command, Command, CommandRunner};
use crate::config::Config;
use crate::node_map::NodeMap;
use crate::viewer::Viewer;
// use crate::shell_commands;

#[allow(dead_code)]
pub struct App {
    node_map: Rc<RefCell<NodeMap>>,
    viewer: Rc<RefCell<Viewer>>,
    command_runner: CommandRunner,
    config: Rc<Config>,
}

#[allow(dead_code, unreachable_code)]
impl App {
    pub fn new(root: PathBuf, config: Config) -> App {
        let config = Rc::new(config);
        let node_map = Rc::new(RefCell::new(NodeMap::new(&root)));
        let viewer = Rc::new(RefCell::new(Viewer::new(node_map.clone(), config.clone())));
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
            
            let mut viewer = self.viewer.borrow_mut();
            viewer.sync()?; // viewerとnode_mapの同期
            viewer.display()?; // 表示

            // 標準入力からコマンドを取得 ------------------------
            let command = match read_command() {
                Err(e) => {
                    viewer.set_console_message(Some(format!("{}", e)));
                    continue;
                }
                Ok(command) => {
                    if command == Command::Quit{
                        break
                    }
                    viewer.set_console_message(None);
                    command
                }
            };

            // command runner内でviewerをborrowするのでこのスコープ内ではドロップ
            std::mem::drop(viewer);

            let result = self.command_runner.run_command(command);

            // if let Err(e) = result {
            //     viewer.set_console_message(Some(format!("{}", e)));
            // }
        }
        return Ok(());
    }
}
