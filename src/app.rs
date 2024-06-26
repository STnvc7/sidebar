use std::env;
use std::path::{Path, PathBuf};

use crossterm::terminal;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use anyhow::Result;

use crate::node;
use crate::viewer;
use crate::viewer::{ConsoleMessageStatus};
use crate::command;
use crate::error::ApplicationError;

pub fn run(path : &Option<String>) -> Result<()>{

    //初期化
    let root : PathBuf;

    match path{
		Some(v) => {let _path = Path::new(v);
                    if _path.exists() == false{
                        return Ok(())
                    }
                    root = _path.canonicalize().unwrap().to_path_buf();},
		None	=> {root = env::current_dir().unwrap()}};

    let mut tree = node::new(&root);
    let mut viewer = viewer::new();
    viewer.set_console_msg(String::from("Press 'h' to see help"), ConsoleMessageStatus::Normal);

    terminal::enable_raw_mode()?;

    //無限ループでキー入力を待ち続ける
    loop{
        let _text = tree.format();
        viewer.set_text(_text);
        viewer.display()?;

        // イベントの取得
        let event = read()?;

        viewer.clean_console();

        let result = match event {
            //キーイベント
            Event::Key(e) => {
                match e{
                    
                    KeyEvent{code:KeyCode::Char('q'),modifiers:_,kind:_,state:_}                =>  {break;},
                    KeyEvent{code:KeyCode::Esc,modifiers:_,kind:_,state:_}                      =>  {break;},
                    KeyEvent{code:KeyCode::Char('p'),modifiers:_,kind:_,state:_}                =>  command::show_path(&tree, &mut viewer),
                    KeyEvent{code:KeyCode::Char('u'),modifiers:_,kind:_,state:_}                =>  command::update(&mut tree, &mut viewer),
                    KeyEvent{code:KeyCode::Char('h'),modifiers:_,kind:_,state:_}                =>  command::help(),
                    KeyEvent{code:KeyCode::Char('n'),modifiers:_,kind:_,state:_}                =>  command::new_file(&mut tree, &mut viewer),
                    KeyEvent{code:KeyCode::Char('N'),modifiers:_,kind:_,state:_}                =>  command::new_folder(&mut tree, &mut viewer),
                    KeyEvent{code:KeyCode::Char('r'),modifiers:_,kind:_,state:_}                =>  command::rename(&mut tree, &mut viewer),
                    KeyEvent{code:KeyCode::Char('m'),modifiers:_,kind:_,state:_}                =>  command::move_to(&mut tree, &mut viewer),
                    KeyEvent{code:KeyCode::Char('c'),modifiers:_,kind:_,state:_}                =>  command::copy(&mut tree, &mut viewer),
                    KeyEvent{code:KeyCode::Enter,modifiers:_,kind:_,state:_}                    =>  command::open_file(&tree, &viewer),
                    KeyEvent{code:KeyCode::Tab,modifiers:_,kind:_,state:_}                      =>  command::open_folder(&mut tree, &viewer),
                    KeyEvent{code:KeyCode::Down,modifiers:KeyModifiers::SHIFT,kind:_,state:_}   =>  command::cursor_jump_down(&mut viewer),
                    KeyEvent{code:KeyCode::Up,modifiers:KeyModifiers::SHIFT,kind:_,state:_}     =>  command::cursor_jump_up(&mut viewer),
                    KeyEvent{code:KeyCode::Left,modifiers:_,kind:_,state:_}                     =>  command::cursor_jump_up(&mut viewer),
                    KeyEvent{code:KeyCode::Right,modifiers:_,kind:_,state:_}                    =>  command::cursor_jump_down(&mut viewer),
                    KeyEvent{code:KeyCode::Down,modifiers:_,kind:_,state:_}                     =>  command::cursor_down(&mut viewer),
                    KeyEvent{code:KeyCode::Up,modifiers:_,kind:_,state:_}                       =>  command::cursor_up(&mut viewer),

                    KeyEvent{code:KeyCode::Char(c),modifiers:_,kind:_,state:_}  => Err(anyhow::anyhow!(ApplicationError::InvalidCommandError(c))),
                    _                                                           => Err(anyhow::anyhow!(ApplicationError::UnacceptableKeyError)),
                }        
            }
            Event::Resize(_, _) => command::resize(&mut viewer),
            _ => {Err(anyhow::anyhow!(ApplicationError::UnacceptableKeyError))}
        };


        match result {
            Ok(_) => {}
            Err(e) => {viewer.set_console_msg(format!("{}",e), ConsoleMessageStatus::Error)}
        }
    }

    terminal::disable_raw_mode().unwrap();

    Ok(())
}