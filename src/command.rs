use std::io::Result;
use std::process::Command;

use crossterm::event::{read, Event, KeyCode};

use crate::node::{Node};
use crate::viewer::{Viewer, ConsoleMessageStatus};


pub enum Commands{
    Quit,
    Up,
    Down,
    JumpUp,
    JumpDown,
    ShowPath,
    Reload,
    NewFile,
    NewFolder,
    OpenFile,
    OpenFolder,
    Resize,
    Help,
}

pub fn cursor_up(viewer : &mut Viewer) -> Result<()> {

    viewer.cursor_up();
    Ok(())
}

pub fn cursor_jump_up(viewer : &mut Viewer) -> Result<()> {

    viewer.cursor_jump_up();
    Ok(())
}

pub fn cursor_down(viewer : &mut Viewer) -> Result<()> {

    viewer.cursor_down();
    Ok(())
}

pub fn cursor_jump_down(viewer : &mut Viewer) -> Result<()> {

    viewer.cursor_jump_down();
    Ok(())
}

pub fn show_path(tree : &Box<Node>, viewer : &mut Viewer) -> Result<()> {

    let route = viewer.get_cursor_route();
    let path = tree.get_path(route).to_string_lossy().into_owned();
    viewer.set_console_msg(path, ConsoleMessageStatus::Normal);
    Ok(())
}

pub fn reload(tree : &mut Box<Node>, viewer : &Viewer) -> Result<()> {

    let mut route = viewer.get_cursor_route();

    loop {
        if route.len() == 0{
            break
        }

        let _path = tree.get_path(route.clone());
        if _path.is_dir() {
            break
        }
        else{
            let _ = route.pop_back().unwrap();
        }
    }

    tree.update(route);
    Ok(())
}

pub fn help(viewer : &mut Viewer) -> Result<()> {

    let _help_msg = String::from("'h' : help, 'q' : quit, 'Enter' : open file or folder, 'p' : show path, 'r' : reset status");
    viewer.set_console_msg(_help_msg, ConsoleMessageStatus::Normal);

    Ok(())
}

pub fn new_file(tree : &Box<Node>, viewer : &mut Viewer) -> Result<()>{

    let route = viewer.get_cursor_route();
    let mut parent_path = tree.get_path(route);

    let mut new_file_name = String::new();
    let console_msg_head = String::from("Enter filename : ");

    loop{
        let _console_msg = format!("{}{}",console_msg_head,new_file_name);
        viewer.set_console_msg(_console_msg, ConsoleMessageStatus::Normal);
        viewer.display()?;

        let _event = read()?;
        match _event{
            Event::Key(_e) =>{
                match _e.code{
                    KeyCode::Char(c) => {new_file_name.push(c);},
                    KeyCode::Enter   => {viewer.clean_console();
                                        break},
                    KeyCode::Backspace  => {if new_file_name.len() != 0 {let _ = new_file_name.pop().unwrap();}}
                   _ => {}
                }
            },
            _ => {}
        }
    }
    if parent_path.is_file(){
        parent_path = (*parent_path.parent().unwrap()).to_path_buf();
    }
    let new_file_path_string = format!("{}/{}", parent_path.to_string_lossy().into_owned(), new_file_name);
    viewer.set_console_msg(new_file_path_string.clone(), ConsoleMessageStatus::Normal);

    let _ = Command::new("touch").arg(new_file_path_string).spawn();

    Ok(())
}

pub fn open_file(tree : &Box<Node>, viewer : &Viewer) -> Result<()> {

    let route = viewer.get_cursor_route();
    let path = tree.get_path(route);
    let _ = Command::new("rsubl").arg(path).spawn();
    Ok(())
}

pub fn open_folder(tree : &mut Box<Node>, viewer : &Viewer) -> Result<()> {

    let _route = viewer.get_cursor_route();
    tree.open_node(_route);
    Ok(())
}

pub fn resize(viewer : &mut Viewer) -> Result<()> {

    viewer.set_terminal_size();
    Ok(())
}