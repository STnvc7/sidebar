use anyhow::{anyhow, Result};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};
use log;
use std::cell::RefCell;
use std::rc::Rc;

use crate::node_map::NodeMap;
use crate::viewer::Viewer;
use crate::config::Config;

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
                return map_key_to_command(e);
            }
            Event::Resize(_, _) => Ok(Command::Resize),
            _ => Err(anyhow!("Unacceptable Key")),
        };
        return command;
    }
}

fn map_key_to_command(key_event: KeyEvent) -> Result<Command> {
    match key_event.code {
        KeyCode::Char('q') | KeyCode::Esc => Ok(Command::Quit),
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
        KeyCode::Left => Ok(Command::JumpUp),
        KeyCode::Right => Ok(Command::JumpDown),
        KeyCode::Down => Ok(Command::Down),
        KeyCode::Up => Ok(Command::Up),
        KeyCode::Char(c) => Err(anyhow!("Invalid Command: {}", c)),
        _ => Err(anyhow!("Invalid Command: {:?}", key_event.code)),
    }
}


pub struct CommandRunner {
    node_map: Rc<RefCell<NodeMap>>,
    viewer: Rc<RefCell<Viewer>>,
    config: Rc<Config>,
}

impl CommandRunner{
    pub fn new(node_map: Rc<RefCell<NodeMap>>, viewer: Rc<RefCell<Viewer>>, config: Rc<Config>) -> CommandRunner{
        CommandRunner{
            node_map: node_map,
            viewer: viewer,
            config: config
        }
    }

    pub fn run_command(&mut self, command: Command) -> Result<()> {
        match command {
            Command::OpenFile => Ok(()),
            Command::OpenFolder => {
                let viewer = self.viewer.borrow_mut();
                let mut node_map = self.node_map.borrow_mut();
                let id = viewer.get_cursor_id();
                node_map.open_and_close_node(&id)
            }
            Command::NewFile => Ok(()),
            Command::NewFolder => Ok(()),
            Command::Copy => Ok(()),
            Command::Move => Ok(()),
            Command::Rename => {
                Ok(())
            },
            Command::Delete => Ok(()),
            Command::ShowPath => Ok(()),
            Command::ExportPath => Ok(()),
            Command::Link => Ok(()),
            Command::Quit => Ok(()),
            Command::Update => {
                let mut node_map = self.node_map.borrow_mut();
                node_map.update()
            }
            Command::Help => Ok(()),
            Command::Resize => {
                let mut viewer = self.viewer.borrow_mut();
                viewer.resize()
            }
            Command::Up => {
                let mut viewer = self.viewer.borrow_mut();
                viewer.cursor_up();
                Ok(())
            }
            Command::Down => {
                let mut viewer = self.viewer.borrow_mut();
                viewer.cursor_down();
                Ok(())
            }
            Command::JumpUp => {
                let mut viewer = self.viewer.borrow_mut();
                viewer.cursor_jump_up()
            }
            Command::JumpDown => {
                let mut viewer = self.viewer.borrow_mut();
                viewer.cursor_jump_down()
            }
        }
    }
}