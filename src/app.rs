use std::env;
use std::io;
use std::path::PathBuf;

use crossterm::terminal;
use crossterm::event::{read, Event, KeyCode};

use crate::node;
use crate::node::{NodeType};
use crate::viewer;
use crate::viewer::{ConsoleMessageStatus};
use crate::command;
use crate::command::Commands;

//無限ループでキー入力を待ち続ける
pub fn run(path : &Option<String>) -> io::Result<()>{
	
    let root = match path{
		Some(v) => {PathBuf::from(v)},
		None	=> {env::current_dir().unwrap()}};

    let mut tree = node::new(&root);
    let mut viewer = viewer::new();


    terminal::enable_raw_mode()?;
    loop{
        let _text = tree.format();
        viewer.set_text(_text);
        viewer.display()?;

        // イベントの取得
        let event = read()?;

        //viewer.clean_console();

        let command = match event {
            //キーイベント
            Event::Key(e) => {
                match e.code{
                    
                    KeyCode::Char('q')  =>  Ok(Commands::Quit),                         //アプリケーションを終了
                    KeyCode::Char('p')  =>  Ok(Commands::ShowPath),                     //選択されているノードのパスを表示
                    KeyCode::Char('r')  =>  Ok(Commands::Reload),                        //リロード
                    KeyCode::Char('h')  =>  Ok(Commands::Help),                         //ヘルプを表示
                    KeyCode::Char('n')  =>  Ok(Commands::NewFile),
                    KeyCode::Enter      => {
                        match viewer.get_cursor_node_type(){
                            NodeType::Folder => Ok(Commands::OpenFolder),               //フォルダをオープン
                            NodeType::File   => Ok(Commands::OpenFile),                 //ファイルを開く(何らかのテキストエディタで)
                        }
                    }
                    KeyCode::Down       =>  Ok(Commands::Down),                         //カーソルを下に移動
                    KeyCode::Up         =>  Ok(Commands::Up),                           //カーソルを上に移動

                    KeyCode::Char(c)    => Err(String::from(format!("{} is invalid command", c))),
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

            Ok(Commands::ShowPath)   => {command::show_path(&tree, &mut viewer)?;}

            Ok(Commands::Reload)     => {command::reload(&mut tree, &viewer)?;}

            Ok(Commands::Help)       => {command::help(&mut viewer)?;}

            Ok(Commands::NewFile)    => {command::new_file(&mut tree, &mut viewer)?;
                                         command::reload(&mut tree, &viewer)?;}

            Ok(Commands::OpenFolder) => {command::open_folder(&mut tree, &viewer)?;}

            Ok(Commands::OpenFile)   => {command::open_file(&tree, &viewer)?;} //TODO!!!!!!!!!!!!!!!

            Ok(Commands::Resize)     => {command::resize(&mut viewer)?;}

            Ok(_)                    => {viewer.set_console_msg(String::from("We're working on!!!"), ConsoleMessageStatus::Error);}

            Err(s) =>  {viewer.set_console_msg(s, ConsoleMessageStatus::Error);}
        };
    }

    terminal::disable_raw_mode()?;

    Ok(())
}