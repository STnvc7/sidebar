use anyhow::{anyhow, Result};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};
use log;
use std::cell::RefCell;
use std::rc::Rc;

use crate::node_map::NodeMap;
use crate::viewer::Viewer;
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
    Help,
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
        KeyCode::Char('h') => Ok(Command::Help),
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
        KeyCode::Char(c) => Err(anyhow!("Invalid Command: {}", c)),
        _ => Err(anyhow!("Invalid Command: {:?}", key_event.code)),
    }
}

// struct Job {
//     command: Command,
//     job_id: u16,
//     node: String
// }
pub struct CommandRunner {
    node_map: Rc<RefCell<NodeMap>>,
    viewer: Rc<RefCell<Viewer>>,
    config: Rc<Config>,
    // jobs: Vec<Job>
}

impl CommandRunner{
    pub fn new(node_map: Rc<RefCell<NodeMap>>, viewer: Rc<RefCell<Viewer>>, config: Rc<Config>) -> CommandRunner{
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
            Command::NewFile => Ok(()),
            Command::NewFolder => Ok(()),
            Command::Copy => Ok(()),
            Command::Move => self.move_(),
            Command::Rename => Ok(()),
            Command::Delete => Ok(()),
            Command::ShowPath => self.show_path(),
            Command::ExportPath => Ok(()),
            Command::Link => Ok(()),
            Command::Quit => Ok(()),
            Command::Update => self.update(),
            Command::Help => Ok(()),
            Command::Resize => self.resize(),
            Command::Up => self.up(),
            Command::Down => self.down(),
            Command::JumpUp => self.jump_up(),
            Command::JumpDown => self.jump_down(),
        }
    }

    fn open_file(&mut self) -> Result<()> {
        let viewer = self.viewer.borrow();
        let node_map = self.node_map.borrow();
        let id = viewer.get_cursor_id();
        let path = node_map.get_path(&id)?;
        shell_command::open_file(&path, &self.config.editor_command)
    }

    fn open_folder(&mut self) -> Result<()> {
        let viewer = self.viewer.borrow_mut();
        let mut node_map = self.node_map.borrow_mut();
        let id = viewer.get_cursor_id();
        node_map.open_and_close_node(&id)
    }

    fn move_(&mut self) -> Result<()> {
        let mut viewer = self.viewer.borrow_mut();
        let node_map = self.node_map.borrow();

        let from_id = viewer.get_cursor_id();
        let from_path = node_map.get_path(&from_id)?;

        viewer.activate_secondly_cursor_idx();
        std::mem::drop(viewer);
        std::mem::drop(node_map);

        loop {
            let command = read_command()?;
            match command {
                Command::Up => {self.up()?},
                Command::Down => {self.down()?},
                Command::JumpUp => {self.jump_up()?},
                Command::JumpDown => {self.jump_down()?},
                Command::OpenFile => break,
                Command::OpenFolder => {self.open_folder()?;}
                Command::Quit => {return Err(anyhow!("input aborted!"))},
                _ => {}
            }
            let mut viewer = self.viewer.borrow_mut();
            viewer.sync()?;
            viewer.display()?;
        }
        let mut viewer = self.viewer.borrow_mut();
        let node_map = self.node_map.borrow();
        let to_id = viewer.get_cursor_id();
        let to_path = node_map.get_path(&to_id)?;
        
        viewer.deactivate_secondly_cursor_idx(); 
        Ok(())
    }

    fn show_path(&mut self) -> Result<()> {
        let node_map = self.node_map.borrow();
        let mut viewer = self.viewer.borrow_mut();
        let id = viewer.get_cursor_id();
        let path = node_map.get_path(&id)?;
        let path_string = path.to_string_lossy().into_owned();
        viewer.set_console_message(path_string);
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        let mut node_map = self.node_map.borrow_mut();
        node_map.update()
    }

    fn resize(&mut self) -> Result<()> {
        let mut viewer = self.viewer.borrow_mut();
        viewer.resize()
    }
    fn up(&mut self) -> Result<()> {
        let mut viewer = self.viewer.borrow_mut();
        viewer.cursor_up();
        Ok(())
    }

    fn down(&mut self) -> Result<()> {
        let mut viewer = self.viewer.borrow_mut();
        viewer.cursor_down();
        Ok(())
    }

    fn jump_up(&mut self) -> Result<()>{
        let mut viewer = self.viewer.borrow_mut();
        viewer.cursor_jump_up()
    }

    fn jump_down(&mut self) -> Result<()>{
        let mut viewer = self.viewer.borrow_mut();
        viewer.cursor_jump_down()
    }
}