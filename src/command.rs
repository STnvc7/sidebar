use std::path::PathBuf;
use std::io::{stdout, Write};

use crossterm::event::{read, Event, KeyEvent, KeyCode};
use crossterm::{cursor, execute, queue, style, terminal};

use duct::cmd;
use anyhow::{Context, Result};

use crate::node::{Node};
use crate::viewer::{Viewer, ConsoleMessageStatus};
use crate::error::ApplicationError;
use crate::color;

pub fn cursor_up(viewer : &mut Viewer) -> Result<()> {

    viewer.cursor_up()?;
    Ok(())
}

pub fn cursor_jump_up(viewer : &mut Viewer) -> Result<()> {

    viewer.cursor_jump_up()?;
    Ok(())
}

pub fn cursor_down(viewer : &mut Viewer) -> Result<()> {

    viewer.cursor_down()?;
    Ok(())
}

pub fn cursor_jump_down(viewer : &mut Viewer) -> Result<()> {

    viewer.cursor_jump_down()?;
    Ok(())
}

pub fn show_path(tree : &Box<Node>, viewer : &mut Viewer) -> Result<()> {

    let route = viewer.get_cursor_route()?;
    let path = tree.get_path(route).to_string_lossy().into_owned();
    viewer.set_console_msg(path, ConsoleMessageStatus::Normal);
    Ok(())
}

pub fn update(tree : &mut Box<Node>, viewer : &Viewer) -> Result<()> {

    let mut route = viewer.get_cursor_route()?;

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

    tree.update_node(route.clone());
    Ok(())
}

pub fn copy() -> Result<()> {
    Ok(())
}

pub fn move_to(tree : &mut Box<Node>, viewer : &mut Viewer) -> Result<()> {

    viewer.activate_secondly_cursor()?;

    let route = viewer.get_cursor_route()?;
    let original_path = tree.get_path(route);
    let file_name_osstr = &original_path.file_name().context("coudln't get filename")?;
    let file_name_str = file_name_osstr.to_str().context("coudln't convert OsStr to &str")?;

    let mut destination = get_path_from_secondly_cursor(tree, viewer)?;

    if destination.is_file() {
        destination = (*destination.parent().unwrap()).to_path_buf();
    }

    let new_file_path_string = format!("{}/{}", destination.to_string_lossy().into_owned(), file_name_str);

    let result = cmd!("mv", original_path, new_file_path_string.clone()).stderr_capture().run();

    match result{
        Ok(_)   => { viewer.set_console_msg(new_file_path_string, ConsoleMessageStatus::Normal);},
        Err(_)  => { return Err(anyhow::anyhow!("coudln't move..."));}
    }

    update(tree, viewer)?;

    viewer.deactivate_secondly_cursor()?;

    Ok(())
}

pub fn help() -> Result<()> {

    queue!(stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0,0), style::Print(color::RESET))?;

    let help_msg = vec![
        "'h'        : Show help",
        "'q'/Esc    : Quit application",
        "↑          : Cursor up",
        "↓          : Curosr down",
        "Shift+↑    : Cursor jump up",
        "Shift+↓    : Curosr jump down",
        "'p'        : Show file/folder path",
        "Enter      : Open file/folder",
        "'n'        : New file",
        "Shift+'n'  : New folder",
        "'r'        : Rename",
        "'m'        : Move",
        ];

    for line in help_msg.iter(){
        queue!(stdout(), style::Print("> "), style::Print(line), cursor::MoveToNextLine(1))?;
    }

    queue!(stdout(), cursor::MoveToNextLine(1), style::Print("Press 'q'/Esc to exit help"), cursor::MoveToNextLine(1))?;

    stdout().flush()?;

    loop{
        let _event = read()?;
        match _event{
            Event::Key(e) =>{
                match e{
                    KeyEvent{code:KeyCode::Char('q'),modifiers:_,kind:_,state:_}  =>  {break;},
                    KeyEvent{code:KeyCode::Esc,modifiers:_,kind:_,state:_}        =>  {break;},
                    _ => {}
                }
            },
            _ => {}
        }
    }

    Ok(())
}

pub fn rename(tree : &mut Box<Node>, viewer : &mut Viewer) -> Result<()> {

    let route = viewer.get_cursor_route()?;
    let original_path = tree.get_path(route);

    let console_msg_head = String::from("Rename to : ");
    let new_name = type_from_console_stdin(viewer, console_msg_head.clone())?;

    if new_name.len() == 0{
        return Err(anyhow::anyhow!("aborted"));
    }
    let new_path_string = format!("{}/{}", (*original_path.parent().unwrap()).to_path_buf().to_string_lossy().into_owned(), new_name);

    let result = cmd!("mv", original_path, new_path_string.clone()).stderr_capture().run();

    match result{
        Ok(_)   => {viewer.set_console_msg(new_path_string, ConsoleMessageStatus::Normal);},
        Err(_)  => { return Err(anyhow::anyhow!("coudln't rename..."));}
    }

    update(tree, viewer)?;

    Ok(())
}

pub fn new_file(tree : &mut Box<Node>, viewer : &mut Viewer) -> Result<()>{

    let route = viewer.get_cursor_route()?;
    let mut parent_path = tree.get_path(route);

    let console_msg_head = String::from("Enter filename : ");
    let new_file_name = type_from_console_stdin(viewer, console_msg_head.clone())?;

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
        Ok(_)   => { viewer.set_console_msg(new_file_path_string, ConsoleMessageStatus::Normal);},
        Err(_)  => { return Err(anyhow::anyhow!("coudln't make file..."));}
    }

    update(tree, viewer)?;

    Ok(())
}

pub fn new_folder(tree : &mut Box<Node>, viewer : &mut Viewer) -> Result<()> {

    let route = viewer.get_cursor_route()?;
    let mut parent_path = tree.get_path(route);

    let console_msg_head = String::from("Enter folder name : ");

    let new_folder_name = type_from_console_stdin(viewer, console_msg_head.clone())?;

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
        Err(_)  => { return Err(anyhow::anyhow!("coudln't make folder...")); }
    }

    update(tree, viewer)?;

    Ok(())
}

pub fn open_file(tree : &Box<Node>, viewer : &mut Viewer) -> Result<()> {

    let route = viewer.get_cursor_route()?;
    let path = tree.get_path(route);
    
    let result = cmd!("rmate", path).stderr_capture().run();

    match result{
        Ok(_)   => { },
        Err(_)  => { return Err(anyhow::anyhow!("coudln't open file..."));}
    }

    Ok(())
}

pub fn open_folder(tree : &mut Box<Node>, viewer : &Viewer) -> Result<()> {

    let _route = viewer.get_cursor_route()?;
    tree.open_node(_route);
    Ok(())
}

pub fn resize(viewer : &mut Viewer) -> Result<()> {

    viewer.set_terminal_size();
    Ok(())
}

//-----------------------------------------------------------------------------
fn type_from_console_stdin(viewer : &mut Viewer, console_msg_head : String) -> Result<String>{
    
    let mut new_name = String::new();

    loop{
        let _console_msg = format!("{}{}",console_msg_head, new_name);
        viewer.set_console_msg(_console_msg, ConsoleMessageStatus::Normal);
        viewer.display()?;

        let _event = read()?;
        match _event{
            Event::Key(_e) =>{
                match _e.code{
                    KeyCode::Char(c) => {new_name.push(c);},
                    KeyCode::Enter   => {viewer.clean_console();
                                         break},
                    KeyCode::Esc     => {new_name.clear();
                                         break}
                    KeyCode::Backspace  => {if new_name.len() != 0 {let _ = new_name.pop().unwrap();}}
                   _ => {}
                }
            },
            _ => {}
        }
    }

    return Ok(new_name)
}

fn get_path_from_secondly_cursor(tree: &Box<Node>, viewer : &mut Viewer) -> Result<PathBuf>{

    let path_result = loop{
        let _event = read()?;
        match _event{
            Event::Key(_e) =>{
                match _e.code{
                    KeyCode::Down    => {let _ = viewer.secondly_cursor_down();},
                    KeyCode::Up      => {let _ = viewer.secondly_cursor_up();},
                    KeyCode::Enter   => {let _route = viewer.get_secondly_cursor_route()?;
                                         break Ok(tree.get_path(_route))},
                    KeyCode::Esc     => {break Err(anyhow::anyhow!("aborted"));}
                   _ => {}
                }
            },
            _ => {}
        }
        viewer.display()?;
    };
    
    return path_result
}