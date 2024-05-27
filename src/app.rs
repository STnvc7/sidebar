use std::env;
use std::io;
use std::process::Command;
use std::path::PathBuf;

use crossterm::terminal;
use crossterm::event::{read, Event, KeyCode};

use crate::node;
use crate::node::NodeType;
use crate::viewer;
use crate::viewer::{ConsoleMessageStatus};

enum Commands{
    Quit,
    Up,
    Down,
    TabUp,
    TabDown,
    ShowPath,
    Reload,
    New,
    OpenFile,
    OpenFolder,
    Resize,
    Help,
}

//無限ループでキー入力を待ち続ける
pub fn run(path : &Option<String>) -> io::Result<()>{
	
    let root = match path{
		Some(v) => {PathBuf::from(v)},
		None	=> {env::current_dir().unwrap()}};

    let mut tree = node::build_tree(&root);
    let mut viewer = viewer::new();

    terminal::enable_raw_mode()?;
    loop{
        let _text = tree.format();
        viewer.set_text(_text);
        viewer.display()?;

        // イベントの取得
        let event = read()?;

        let command = match event {
            //キーイベント
            Event::Key(e) => {
                match e.code{
                    
                    KeyCode::Char('q')  =>  Ok(Commands::Quit),                         //アプリケーションを終了
                    KeyCode::Char('p')  =>  Ok(Commands::ShowPath),                     //選択されているノードのパスを表示
                    KeyCode::Char('r')  =>  Ok(Commands::Reload),                        //リセット
                    KeyCode::Char('h')  =>  Ok(Commands::Help),                         //ヘルプを表示
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

            Ok(Commands::Up)         => {viewer.cursor_up();}

            Ok(Commands::Down)       => {viewer.cursor_down();}

            Ok(Commands::ShowPath)   => {let _route = viewer.get_cursor_route();
                                         let _path = tree.get_path(_route).to_string_lossy().into_owned();
                                         viewer.set_console_msg(_path, ConsoleMessageStatus::Normal);}

            Ok(Commands::Reload)      => {let _route = viewer.get_cursor_route();
                                         tree.update(_route)}

            Ok(Commands::Help)       => {let _help_msg = String::from("'h' : help, 'q' : quit, 'Enter' : open file or folder, 'p' : show path, 'r' : reset status");
                                         viewer.set_console_msg(_help_msg, ConsoleMessageStatus::Normal);}

            Ok(Commands::OpenFolder) => {let _route = viewer.get_cursor_route();
                                         tree.open_node(_route);}

            Ok(Commands::OpenFile)   => {let _route = viewer.get_cursor_route();
                                         let _path = tree.get_path(_route);
                                         let _ = Command::new("rsubl").arg(_path).spawn();} //TODO!!!!!!!!!!!!!!!

            Ok(Commands::Resize)     => {viewer.set_terminal_size();}

            Ok(_)                    => {viewer.set_console_msg(String::from("We're working on!!!"), ConsoleMessageStatus::Error);}

            Err(s) =>  {viewer.set_console_msg(s, ConsoleMessageStatus::Error);}
        };
    }

    terminal::disable_raw_mode()?;

    Ok(())
}
