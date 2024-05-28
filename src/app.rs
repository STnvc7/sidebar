use std::env;
use std::io;
use std::path::{Path, PathBuf};

use crossterm::terminal;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::node;
use crate::node::{NodeType};
use crate::viewer;
use crate::viewer::{ConsoleMessageStatus};
use crate::command;
use crate::command::Commands;

//無限ループでキー入力を待ち続ける
pub fn run(path : &Option<String>) -> io::Result<()>{

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


    terminal::enable_raw_mode().unwrap();
    loop{
        let _text = tree.format();
        viewer.set_text(_text);
        viewer.display();

        // イベントの取得
        let event = read().unwrap();

        //viewer.clean_console();

        let command = match event {
            //キーイベント
            Event::Key(e) => {
                match e{
                    
                    KeyEvent{code:KeyCode::Char('q'),modifiers:_,kind:_,state:_}  =>  Ok(Commands::Quit),                         //アプリケーションを終了
                    KeyEvent{code:KeyCode::Char('p'),modifiers:_,kind:_,state:_}  =>  Ok(Commands::ShowPath),                     //選択されているノードのパスを表示
                    KeyEvent{code:KeyCode::Char('u'),modifiers:_,kind:_,state:_}  =>  Ok(Commands::Update),                        //リロード
                    KeyEvent{code:KeyCode::Char('h'),modifiers:_,kind:_,state:_}  =>  Ok(Commands::Help),                         //ヘルプを表示
                    KeyEvent{code:KeyCode::Char('n'),modifiers:_,kind:_,state:_}  =>  Ok(Commands::NewFile),
                    KeyEvent{code:KeyCode::Enter,modifiers:_,kind:_,state:_}      => {
                        match viewer.get_cursor_node_type(){
                            NodeType::Folder => Ok(Commands::OpenFolder),               //フォルダをオープン
                            NodeType::File   => Ok(Commands::OpenFile),                 //ファイルを開く(何らかのテキストエディタで)
                        }
                    }
                    KeyEvent{code:KeyCode::Down,modifiers:KeyModifiers::SHIFT,kind:_,state:_}       =>  Ok(Commands::JumpDown),
                    KeyEvent{code:KeyCode::Up,modifiers:KeyModifiers::SHIFT,kind:_,state:_}       =>  Ok(Commands::JumpUp),
                    KeyEvent{code:KeyCode::Down,modifiers:_,kind:_,state:_}       =>  Ok(Commands::Down),                         //カーソルを下に移動
                    KeyEvent{code:KeyCode::Up,modifiers:_,kind:_,state:_}         =>  Ok(Commands::Up),                           //カーソルを上に移動

                    KeyEvent{code:KeyCode::Char(c),modifiers:_,kind:_,state:_}    => Err(String::from(format!("{} is invalid command", c))),
                    _ => Err(String::from("no covered key")),
                }        
            }
            Event::Resize(_, _) => Ok(Commands::Resize),
            _ => {Err(String::from("cannot accept keys..."))}
        };

        match command{
            Ok(Commands::Quit)       => {break;}
            Ok(Commands::Up)         => {command::cursor_up(&mut viewer)?;}
            Ok(Commands::Down)       => {command::cursor_down(&mut viewer)?;}
            Ok(Commands::JumpUp)     => {command::cursor_jump_up(&mut viewer)?;}
            Ok(Commands::JumpDown)   => {command::cursor_jump_down(&mut viewer)?;}
            Ok(Commands::ShowPath)   => {command::show_path(&tree, &mut viewer)?;}
            Ok(Commands::Update)     => {command::update(&mut tree, &viewer)?;}
            Ok(Commands::Help)       => {command::help(&mut viewer)?;}
            Ok(Commands::NewFile)    => {command::new_file(&mut tree, &mut viewer)?;
                                         command::update(&mut tree, &viewer)?;}
            Ok(Commands::OpenFolder) => {command::open_folder(&mut tree, &viewer)?;}
            Ok(Commands::OpenFile)   => {command::open_file(&tree, &viewer)?;} //TODO!!!!!!!!!!!!!!!
            Ok(Commands::Resize)     => {command::resize(&mut viewer)?;}
            Ok(_)                    => {viewer.set_console_msg(String::from("We're working on!!!"), ConsoleMessageStatus::Error);}
            Err(s) =>  {viewer.set_console_msg(s, ConsoleMessageStatus::Error);}
        };
    }

    terminal::disable_raw_mode().unwrap();

    Ok(())
}