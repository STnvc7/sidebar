#![allow(unused_imports)]

use anyhow::{anyhow, Result};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};
use log;
use std::sync::{Mutex, Arc};
use std::fs;

use crate::node_map::NodeMap;
use crate::viewer::{Viewer, ConsoleMessageStatus};
use crate::config::Config;
use crate::shell_command;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    OpenFile,
    OpenFolder,
    NewFile,
    NewFolder,
    Copy,
    Move,
    Rename,
    Delete,
    ShowPath,
    ExportPath,
    Link,
    Quit,
    Update,
    Shell,
    Resize,
    Up,
    Down,
    JumpUp,
    JumpDown,
}

pub fn read_command() -> Result<Command> {
    // キーが押されたときと話されたときでイベントが送信されるので，押されたときのみ受理
    loop {
        let event = read()?;

        let command = match event {
            Event::Key(e) => {
                if e.kind == KeyEventKind::Release {
                    continue;
                }
                key_to_command(e)
            }
            Event::Resize(_, _) => Ok(Command::Resize),
            _ => Err(anyhow!(" > Unacceptable Key")),
        };
        return command;
    }
}

fn key_to_command(key_event: KeyEvent) -> Result<Command> {
    match key_event.code {
        KeyCode::Char('p') => Ok(Command::ShowPath),
        KeyCode::Char('u') => Ok(Command::Update),
        KeyCode::Char('s') => Ok(Command::Shell),
        KeyCode::Char('n') => Ok(Command::NewFile),
        KeyCode::Char('N') => Ok(Command::NewFolder),
        KeyCode::Char('r') => Ok(Command::Rename),
        KeyCode::Char('m') => Ok(Command::Move),
        KeyCode::Char('c') => Ok(Command::Copy),
        KeyCode::Char('e') => Ok(Command::ExportPath),
        KeyCode::Char('l') => Ok(Command::Link),
        KeyCode::Enter => Ok(Command::OpenFile),
        KeyCode::Tab => Ok(Command::OpenFolder),
        KeyCode::Backspace => Ok(Command::Delete),
        KeyCode::Esc => Ok(Command::Quit),
        KeyCode::Left => Ok(Command::JumpUp),
        KeyCode::Right => Ok(Command::JumpDown),
        KeyCode::Down => Ok(Command::Down),
        KeyCode::Up => Ok(Command::Up),
        KeyCode::Char(c) => Err(anyhow!(" > Invalid Command: {}", c)),
        _ => Err(anyhow!(" > Invalid Command: {:?}", key_event.code)),
    }
}

// =====================================================================================
// struct Job {
//     command: Command,
//     job_id: u16,
//     node: String
// }
pub struct CommandRunner {
    node_map: Arc<Mutex<NodeMap>>,
    viewer: Arc<Mutex<Viewer>>,
    config: Arc<Config>,
    // jobs: Vec<Job>
}

impl CommandRunner{
    pub fn new(node_map: Arc<Mutex<NodeMap>>, viewer: Arc<Mutex<Viewer>>, config: Arc<Config>) -> CommandRunner{
        CommandRunner{
            node_map: node_map,
            viewer: viewer,
            config: config,
            // jobs: Vec::new()
        }
    }

    pub fn run_command(&mut self, command: Command) -> Result<()> {
        match command {
            Command::OpenFile => self.open_file(),
            Command::OpenFolder => self.open_folder(),
            Command::NewFile => self.new_file(),
            Command::NewFolder => self.new_folder(),
            Command::Copy => self.copy(),
            Command::Move => {
                self.move_()?;
                self.update()
            }
            Command::Rename => self.rename(),
            Command::Delete => self.delete(),
            Command::ShowPath => self.show_path(),
            Command::ExportPath => self.export_path(),
            Command::Link => self.link(),
            Command::Quit => Ok(()),
            Command::Update => self.update(),
            Command::Shell => Ok(()),
            Command::Resize => self.resize(),
            Command::Up => self.up(),
            Command::Down => self.down(),
            Command::JumpUp => self.jump_up(),
            Command::JumpDown => self.jump_down(),
        }
    }

    // エディタでファイルを開く -------------------------
    fn open_file(&mut self) -> Result<()> {
        let viewer = self.viewer.lock().unwrap();
        let node_map = self.node_map.lock().unwrap();
        let id = viewer.get_cursor_id();
        let path = node_map.get_path(&id)?;
        shell_command::open_file(&path, &self.config.editor_command)
    }

    // フォルダを展開 ------------------------------
    fn open_folder(&mut self) -> Result<()> {
        let viewer = self.viewer.lock().unwrap();
        let mut node_map = self.node_map.lock().unwrap();
        let id = viewer.get_cursor_id();
        node_map.open_and_close_node(&id)
    }

    //新しいファイルを作成 --------------------------
    fn new_file(&mut self) -> Result<()> {
        Ok(())
    }

    // 新しいディレクトリを作成 -----------------------
    fn new_folder(&mut self) -> Result<()> {
        Ok(())
    }

    // コピー ------------------------------------
    fn copy(&mut self) -> Result<()> {
        Ok(())
    }

    // ファイルの移動 ------------------------------
    fn move_(&mut self) -> Result<()> {
        let mut viewer = self.viewer.lock().unwrap();
        let node_map = self.node_map.lock().unwrap();

        // 移動元のパスを取得
        let from_id = viewer.get_cursor_id();
        let from_path = node_map.get_path(&from_id)?;

        viewer.activate_secondly_cursor();

        // 一旦RefCellのやつをドロップ
        std::mem::drop(viewer);
        std::mem::drop(node_map);

        // 移動先を入力する
        loop {
            let command = read_command();
            match command {
                Ok(Command::Up) => {self.up()?},
                Ok(Command::Down) => {self.down()?},
                Ok(Command::JumpUp) => {self.jump_up()?},
                Ok(Command::JumpDown) => {self.jump_down()?},
                Ok(Command::OpenFile) => {
                    let mut viewer = self.viewer.lock().unwrap();
                    viewer.deactivate_secondly_cursor(); 
                    break
                },
                Ok(Command::OpenFolder) => {self.open_folder()?;}
                Ok(Command::Resize) => {self.resize()?;}
                Ok(Command::Quit) => {
                    let mut viewer = self.viewer.lock().unwrap();
                    viewer.deactivate_secondly_cursor(); 
                    return Err(anyhow!("> Input aborted!"))
                },
                _ => {}
            }
            let mut viewer = self.viewer.lock().unwrap();
            viewer.set_console_message(format!("> Move from: {}", from_path.to_string_lossy()), ConsoleMessageStatus::Info);
            viewer.sync()?;
            viewer.display()?;
        };


        let node_map = self.node_map.lock().unwrap();
        let mut viewer = self.viewer.lock().unwrap();
        
        // 移動先を取得 --------------------
        let to_id = viewer.get_cursor_id();
        let mut to_path = match node_map.get_path(&to_id){
            Ok(path) => {
                if path.is_dir() == false {
                    path.parent().unwrap().to_path_buf()
                }else {
                    path
                }
            }
            Err(e) => {return Err(e)}
        };
        // 
        let file_name = from_path.file_name().unwrap();
        to_path.push(file_name);

        // 実行
        fs::rename(&from_path, &to_path)?;
        viewer.set_console_message(format!("> Move to: {}", to_path.to_string_lossy()), ConsoleMessageStatus::Notify);
        Ok(())
    }

    // 名前の変更 ---------------------------------------------------
    fn rename(&mut self) -> Result<()> {
        Ok(())
    }

    // 削除 --------------------------------------------------------
    fn delete(&mut self) -> Result<()> {
        Ok(())
    }

    // パスを表示 ----------------------------------------------------
    fn show_path(&mut self) -> Result<()> {
        let node_map = self.node_map.lock().unwrap();
        let mut viewer = self.viewer.lock().unwrap();
        let id = viewer.get_cursor_id();
        let path = node_map.get_path(&id)?;
        let path_string = path.to_string_lossy().into_owned();
        viewer.set_console_message(path_string, ConsoleMessageStatus::Info);
        Ok(())
    }

    // パスを環境変数にエクスポート -------------------------------------
    fn export_path(&mut self) -> Result<()> {
        let node_map = self.node_map.lock().unwrap();
        let mut viewer = self.viewer.lock().unwrap();
        let id = viewer.get_cursor_id();
        let path = node_map.get_path(&id)?;
        let path_string = path.to_string_lossy().into_owned();
        // shell_command::export(path_string, self.cfg.export_variable_name);
        viewer.set_console_message(path_string, ConsoleMessageStatus::Notify);
        Ok(())
    }

    // シンボリックリンクを作成 ----------------------------------------
    fn link(&mut self) -> Result<()> {
        Ok(())
    }

    // ツリーを更新 -------------------------------------------------
    fn update(&mut self) -> Result<()> {
        let mut node_map = self.node_map.lock().unwrap();
        node_map.update()
    }

    // 画面のリサイズ -----------------------------------------------
    fn resize(&mut self) -> Result<()> {
        let mut viewer = self.viewer.lock().unwrap();
        viewer.resize()
    }

    // カーソルを上へ -----------------------------------------------
    fn up(&mut self) -> Result<()> {
        let mut viewer = self.viewer.lock().unwrap();
        viewer.cursor_up();
        Ok(())
    }

    // カーソルを下へ -----------------------------------------------
    fn down(&mut self) -> Result<()> {
        let mut viewer = self.viewer.lock().unwrap();
        viewer.cursor_down();
        Ok(())
    }

    // カーソルを上の階層へ -----------------------------------------
    fn jump_up(&mut self) -> Result<()>{
        let mut viewer = self.viewer.lock().unwrap();
        viewer.cursor_jump_up()
    }

    // カーソルを下の階層へ ------------------------------------------
    fn jump_down(&mut self) -> Result<()>{
        let mut viewer = self.viewer.lock().unwrap();
        viewer.cursor_jump_down()
    }
}