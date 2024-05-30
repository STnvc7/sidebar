use std::io::Result;

use crossterm::event::{read, Event, KeyCode};
use duct::cmd;

use crate::node::{Node};
use crate::viewer::{Viewer, ConsoleMessageStatus};


pub enum Commands{
    Quit,
    Up,
    Down,
    JumpUp,
    JumpDown,
    ShowPath,
    Update,
    Rename,
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

pub fn update(tree : &mut Box<Node>, viewer : &Viewer) -> Result<()> {

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

    //new_file_nameにファイル名を
    loop{
        let _console_msg = format!("{}{}",console_msg_head,new_file_name);
        viewer.set_console_msg(_console_msg, ConsoleMessageStatus::Normal);
        viewer.display();

        let _event = read().unwrap();
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

    if new_file_name.len() == 0{
        return Ok(())
    }

    //選択されているノードがファイルだったら，その親ノードの子にファイルを作成
    if parent_path.is_file(){
        parent_path = (*parent_path.parent().unwrap()).to_path_buf();
    }
    let new_file_path_string = format!("{}/{}", parent_path.to_string_lossy().into_owned(), new_file_name);

    let result = cmd!("touch", new_file_path_string.clone()).stderr_capture().run();

    match result{
        Ok(_)   => {viewer.set_console_msg(new_file_path_string, ConsoleMessageStatus::Normal);},
        Err(_)  => {viewer.set_console_msg("coudln't make file...".to_string(), ConsoleMessageStatus::Error);}
    }

    Ok(())
}

pub fn new_folder(tree : &Box<Node>, viewer : &mut Viewer) -> Result<()> {

    let route = viewer.get_cursor_route();
    let mut parent_path = tree.get_path(route);

    let mut new_folder_name = String::new();
    let console_msg_head = String::from("Enter folder name : ");

    loop{
        let _console_msg = format!("{}{}",console_msg_head,new_folder_name);
        viewer.set_console_msg(_console_msg, ConsoleMessageStatus::Normal);
        viewer.display();

        let _event = read().unwrap();
        match _event{
            Event::Key(_e) =>{
                match _e.code{
                    KeyCode::Char(c) => {new_folder_name.push(c);},
                    KeyCode::Enter   => {viewer.clean_console();
                                        break},
                    KeyCode::Backspace  => {if new_folder_name.len() != 0 {let _ = new_folder_name.pop().unwrap();}}
                   _ => {}
                }
            },
            _ => {}
        }
    }

    if new_folder_name.len() == 0{
        return Ok(())
    }

    //選択されているノードがファイルだったら，その親ノードの子にファイルを作成
    if parent_path.is_file(){
        parent_path = (*parent_path.parent().unwrap()).to_path_buf();
    }
    let new_folder_path_string = format!("{}/{}", parent_path.to_string_lossy().into_owned(), new_folder_name);

    let result = cmd!("mkdir", new_folder_path_string.clone()).stderr_capture().run();

    match result{
        Ok(_)   => {viewer.set_console_msg(new_folder_path_string, ConsoleMessageStatus::Normal);},
        Err(_)  => {viewer.set_console_msg("coudln't make folder...".to_string(), ConsoleMessageStatus::Error);}
    }

    Ok(())
}

pub fn open_file(tree : &Box<Node>, viewer : &mut Viewer) -> Result<()> {

    let route = viewer.get_cursor_route();
    let path = tree.get_path(route);
    
    let result = cmd!("rmat", path).stderr_capture().run();

    match result{
        Ok(_)   => { },
        Err(_)  => { viewer.set_console_msg("coudln't open file...".to_string(), ConsoleMessageStatus::Error);}
    }

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