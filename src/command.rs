#![allow(unused_imports)]

use anyhow::{anyhow, Result};
use duct::cmd;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};
use log;
use std::sync::{Mutex, Arc};
use std::fs;
use std::path::PathBuf;
use fs_extra;

use crate::node_map::NodeMap;
use crate::viewer::{Viewer, ConsoleMessageStatus};
use crate::config::Config;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Enter,
    OpenFolder,
    NewFile,
    NewFolder,
    Copy,
    Move,
    Rename,
    Delete,
    ShowPath,
    Sync,
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
            _ => Err(anyhow!("Unacceptable Key")),
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
        KeyCode::Char('y') => Ok(Command::Sync),
        KeyCode::Char('l') => Ok(Command::Link),
        KeyCode::Enter => Ok(Command::Enter),
        KeyCode::Tab => Ok(Command::OpenFolder),
        KeyCode::Backspace => Ok(Command::Delete),
        KeyCode::Esc => Ok(Command::Quit),
        KeyCode::Left => Ok(Command::JumpUp),
        KeyCode::Right => Ok(Command::JumpDown),
        KeyCode::Down => Ok(Command::Down),
        KeyCode::Up => Ok(Command::Up),
        KeyCode::Char(c) => Err(anyhow!("Invalid Command: {}", c)),
        _ => Err(anyhow!("Invalid Command: {:?}", key_event.code)),
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
            Command::Enter => {self.open_file()?;},
            Command::OpenFolder => {self.open_folder()?;},
            Command::NewFile => {self.new_file()?;},
            Command::NewFolder => {self.new_folder()?;},
            Command::Copy => {self.copy()?;},
            Command::Move => {self.move_()?;},
            Command::Rename => {self.rename()?;},
            Command::Delete => {self.delete()?;},
            Command::ShowPath => {self.show_path()?;},
            Command::Sync => {self.sync()?;},
            Command::Link => {self.link()?;},
            Command::Quit => {},
            Command::Update => {self.update()?;},
            Command::Shell => {},
            Command::Resize => {self.resize()?;},
            Command::Up => {self.up()?;},
            Command::Down => {self.down()?},
            Command::JumpUp => {self.jump_up()?;},
            Command::JumpDown => {self.jump_down()?;},
        }
        return Ok(())

    }

    //入力用関数 ==============================================================================
    fn select_directory_by_secondoly_cursor(&mut self, message: String) -> Result<PathBuf> {
        loop {
            let mut viewer = self.viewer.lock().unwrap();
            viewer.activate_secondly_cursor();
            viewer.set_console_message(message.clone(), ConsoleMessageStatus::Info);
            viewer.sync()?;
            viewer.display()?;
            std::mem::drop(viewer);

            match read_command() {
                Ok(Command::Up) => {self.up()?},
                Ok(Command::Down) => {self.down()?},
                Ok(Command::JumpUp) => {self.jump_up()?},
                Ok(Command::JumpDown) => {self.jump_down()?},
                Ok(Command::Enter) => {
                    let mut viewer = self.viewer.lock().unwrap();
                    viewer.deactivate_secondly_cursor(); 
                    break
                },
                Ok(Command::OpenFolder) => {self.open_folder()?;}
                Ok(Command::Resize) => {self.resize()?;}
                Ok(Command::Quit) => {
                    let mut viewer = self.viewer.lock().unwrap();
                    viewer.deactivate_secondly_cursor(); 
                    return Err(anyhow!("Input aborted!"))
                },
                _ => {}
            }
        };


        let node_map = self.node_map.lock().unwrap();
        let viewer = self.viewer.lock().unwrap();
        
        // 移動先を取得 --------------------
        let to_id = viewer.get_cursor_id();
        let to_path = match node_map.get_path(&to_id){
            Ok(path) => {
                if path.is_dir() == false {
                    path.parent().unwrap().to_path_buf()
                }else {
                    path
                }
            }
            Err(e) => {return Err(e)}
        };

        return Ok(to_path)
    }

    fn input(&mut self, message: String) -> Result<String> {
        let mut buf = String::new();
        loop {
            let mut viewer = self.viewer.lock().unwrap();
            viewer.set_console_message(format!("{}: {}", message, buf), ConsoleMessageStatus::Info);
            viewer.display()?;

            match read()? {
                Event::Key(k) => {
                    if k.kind == KeyEventKind::Release {
                        continue
                    }
                    match k.code {
                        KeyCode::Char(c) => {buf.push(c);},
                        KeyCode::Backspace => {if buf.len() != 0 {let _ = buf.pop().unwrap();}}
                        KeyCode::Esc => {return Err(anyhow!("Input aborted!"))}
                        KeyCode::Enter => break,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        Ok(buf)
    } 

    fn confirm(&mut self, message: String) -> Result<()> {
        let mut viewer = self.viewer.lock().unwrap();
        viewer.set_console_message(
            format!("{}: Yes->ENTER / No->OTHER", message), 
            ConsoleMessageStatus::Error
        );
        viewer.display()?;
        
        match read_command() {
            Ok(Command::Enter) => return Ok(()),
            _ => return Err(anyhow!("Cancelled!"))
        }
    }

    fn confirm_overwrite(&mut self, path: &PathBuf) -> Result<()> {
        if path.exists() {
            if self.config.skip_exist {
                return Err(anyhow!("{:?} is already existed!", path))
            }
            self.confirm(format!("Overwrite?"))?;
        }
        Ok(())
    }
    // ===================================================================================

    fn get_cursor_path(&self) -> Result<PathBuf> {
        let viewer = self.viewer.lock().unwrap();
        let node_map = self.node_map.lock().unwrap();
        let id = viewer.get_cursor_id();
        let path = node_map.get_path(&id)?;
        return Ok(path)
    }

    // ↓ コマンドたち ↓ ======================================================================

    // エディタでファイルを開く -------------------------
    fn open_file(&mut self) -> Result<()> {
        let path = self.get_cursor_path()?;
        if path.is_file() == false {
            return Err(anyhow!("Not file"))
        }
        cmd!(&self.config.editor_command, path).stderr_capture().run()?;
        Ok(())
    }

    // フォルダを展開 ------------------------------
    fn open_folder(&mut self) -> Result<()> {
        let viewer = self.viewer.lock().unwrap();
        let mut node_map = self.node_map.lock().unwrap();
        let id = viewer.get_cursor_id();
        let path = node_map.get_path(&id)?;

        if path.is_dir() == false {
            return Err(anyhow!("Not folder"))
        }

        node_map.open_and_close_node(&id)
    }

    //新しいファイルを作成 --------------------------
    fn new_file(&mut self) -> Result<()> {
        let file_name = self.input(String::from("Enter"))?;
        let path = self.get_cursor_path()?;

        let new_file_path = if path.is_dir() {
            path.join(&file_name)
        } else {
            match path.parent(){
                Some(p) => p.to_path_buf().join(&file_name),
                None => return Err(anyhow!("Invalid path"))
            }
        };

        // 上書きしますか
        self.confirm_overwrite(&new_file_path)?;
        
        fs::File::create(&new_file_path)?;
        let mut viewer = self.viewer.lock().unwrap();
        viewer.set_console_message(
            format!("New file: {}", new_file_path.to_string_lossy()), 
            ConsoleMessageStatus::Notify
        );
        Ok(())
    }

    // 新しいディレクトリを作成 -----------------------
    fn new_folder(&mut self) -> Result<()> {
        let dir_name = self.input(String::from("Enter"))?;
        let path = self.get_cursor_path()?;

        let new_dir_path = if path.is_dir() {
            path.join(&dir_name)
        } else {
            match path.parent(){
                Some(p) => p.to_path_buf().join(&dir_name),
                None => return Err(anyhow!("Invalid path"))
            }
        };

        // 上書きしますか
        self.confirm_overwrite(&new_dir_path)?;
        
        fs::create_dir(&new_dir_path)?;
        let mut viewer = self.viewer.lock().unwrap();
        viewer.set_console_message(
            format!("New folder: {}", new_dir_path.to_string_lossy()), 
            ConsoleMessageStatus::Notify
        );
        Ok(())
    }

    // コピー ------------------------------------
    fn copy(&mut self) -> Result<()> {
        let from_path = self.get_cursor_path()?;

        // 移動先を入力する
        let to_path_dir = self.select_directory_by_secondoly_cursor(format!("Copy from: {}", from_path.to_string_lossy()))?;
        let file_name = from_path.file_name().unwrap();
        let to_path = to_path_dir.join(file_name);


        // 移動先のパスが既に存在しているとき
        self.confirm_overwrite(&to_path)?;

        // 実行 (上書きするかは確認しているので上書きオプションはtrue)
        if from_path.is_dir() {
            let mut option = fs_extra::dir::CopyOptions::new();
            option.overwrite = true;
            fs_extra::dir::copy(&from_path, &to_path, &option)?;
        } else {
            let mut option = fs_extra::file::CopyOptions::new();
            option.overwrite = true;
            fs_extra::file::copy(&from_path, &to_path, &option)?;
        }

        let mut viewer = self.viewer.lock().unwrap();
        viewer.set_console_message(format!("Copy to: {}", to_path.to_string_lossy()), ConsoleMessageStatus::Notify);
        Ok(())
    }

    // ファイルの移動 ------------------------------
    fn move_(&mut self) -> Result<()> {
        let from_path = self.get_cursor_path()?;

        // 移動先を入力する
        let to_path_dir = self.select_directory_by_secondoly_cursor(format!("Move from: {}", from_path.to_string_lossy()))?;
        let file_name = from_path.file_name().unwrap();
        let to_path = to_path_dir.join(file_name);

        // 移動先のパスが既に存在しているかどうか
        self.confirm_overwrite(&to_path)?;

        // 実行 (上書きするかは確認しているので強制的に上書き)
        if from_path.is_dir() {
            let mut option = fs_extra::dir::CopyOptions::new();
            option.overwrite = true;
            fs_extra::dir::move_dir(&from_path, &to_path, &option)?;
        } else {
            let mut option = fs_extra::file::CopyOptions::new();
            option.overwrite = true;
            fs_extra::file::move_file(&from_path, &to_path, &option)?;
        }

        let mut viewer = self.viewer.lock().unwrap();
        viewer.set_console_message(format!("Move to: {}", to_path.to_string_lossy()), ConsoleMessageStatus::Notify);
        Ok(())
    }

    // 名前の変更 ---------------------------------------------------
    fn rename(&mut self) -> Result<()> {
        let new_name = self.input(String::from("Enter"))?;
        let from_path = self.get_cursor_path()?;

        let new_path = match from_path.parent(){
            Some(p) => p.to_path_buf().join(&new_name),
            None => return Err(anyhow!("Invalid path"))
        };

        self.confirm_overwrite(&new_path)?;
        
        fs::rename(&from_path, &new_path)?;
        let mut viewer = self.viewer.lock().unwrap();
        viewer.set_console_message(
            format!("Renamed: {}", new_path.to_string_lossy()), 
            ConsoleMessageStatus::Notify
        );
        Ok(())
    }

    // 削除 --------------------------------------------------------
    fn delete(&mut self) -> Result<()> {
        let path = self.get_cursor_path()?;
        self.confirm(format!("Remove {}?", path.to_string_lossy()))?;

        // 削除
        if path.is_dir() {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
        let mut viewer = self.viewer.lock().unwrap();
        viewer.set_console_message(
            format!("Removed: {}", path.to_string_lossy()), 
            ConsoleMessageStatus::Notify
        );
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

    // 別のシェルのカレントディレクトリを同期 -------------------------------------
    fn sync(&mut self) -> Result<()> {
        // let node_map = self.node_map.lock().unwrap();
        // let mut viewer = self.viewer.lock().unwrap();
        // let id = viewer.get_cursor_id();
        // let path = node_map.get_path(&id)?;
        // // shell_command::export(path_string, self.cfg.export_variable_name);
        // viewer.set_console_message(format!("{}", path.to_string_lossy()), ConsoleMessageStatus::Notify);
        // Ok(())
        Ok(())
    }

    // シンボリックリンクを作成 ----------------------------------------
    fn link(&mut self) -> Result<()> {
        #[cfg(unix)] // Unix系システム（Linux、macOSなど）
        fn create_symlink(source: &PathBuf, dest: &PathBuf) -> Result<()> {
            use std::os::unix::fs::symlink;
            symlink(source, dest)?;
            Ok(())
        }

        #[cfg(windows)] // Windowsシステム用
        fn create_symlink(source: &PathBuf, dest: &PathBuf) -> Result<()> {
            use std::os::windows::fs::symlink_file;
            symlink_file(source, dest)?;
            Ok(())
        }

        let source = self.get_cursor_path()?;

        // 移動先を入力する
        let dest_dir = self.select_directory_by_secondoly_cursor(format!("Link {}", source.to_string_lossy()))?;
        let file_name = source.file_name().unwrap();
        let dest = dest_dir.join(file_name);

        // 移動先のパスが既に存在しているかどうか
        self.confirm_overwrite(&dest)?;

        create_symlink(&source, &dest)?;
        let mut viewer = self.viewer.lock().unwrap();
        viewer.set_console_message(format!("Linked: {}", dest.to_string_lossy()), ConsoleMessageStatus::Notify);
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