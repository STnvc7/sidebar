use std::env;
use std::io;
use std::process::Command;

use crossterm::terminal;
use crossterm::event::{read, Event, KeyCode};

use crate::node;
use crate::node::NodeType;
use crate::text_line::{ConsoleMessageStatus};
use crate::text_line;

enum Commands{
    Quit,
    Up,
    Down,
    ShowPath,
    Reset,
    OpenFile,
    OpenFolder,
    Resize,
}

//無限ループでキー入力を待ち続ける
pub fn run() -> io::Result<()>{

    let root = env::current_dir().unwrap();

    let mut tree = node::build_tree(&root);
    let mut text_line = text_line::new();

    terminal::enable_raw_mode()?;
    loop{
        let _text = tree.format();
        text_line.set_text(_text);
        text_line.display()?;

        // イベントの取得
        let event = read()?;

        let command = match event {
            //キーイベント
            Event::Key(e) => {
                match e.code{
                    
                    KeyCode::Char('q')  =>  Ok(Commands::Quit),                         //アプリケーションを終了
                    KeyCode::Char('p')  =>  Ok(Commands::ShowPath),                     //選択されているノードのパスを表示
                    KeyCode::Char('r')  =>  Ok(Commands::Reset),                        //リセット
                    KeyCode::Enter      => {
                        match text_line.get_cursor_node_type(){
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

            Ok(Commands::Up)         => {text_line.cursor_up();}

            Ok(Commands::Down)       => {text_line.cursor_down();}

            Ok(Commands::ShowPath)   => {let _route = text_line.get_cursor_route();
                                         let _path = tree.get_path(_route).to_string_lossy().into_owned();
                                         text_line.set_console_msg(_path, ConsoleMessageStatus::Normal);}

            Ok(Commands::Reset)      => {tree = node::build_tree(&root);
                                         text_line = text_line::new();}

            Ok(Commands::OpenFolder) => {let _route = text_line.get_cursor_route();
                                         tree.open_node(_route);}

            Ok(Commands::OpenFile)   => {let _route = text_line.get_cursor_route();
                                         let _path = tree.get_path(_route);
                                         let _ = Command::new("rsubl").arg(_path).spawn();} //TODO!!!!!!!!!!!!!!!

            Ok(Commands::Resize)     => {text_line.set_terminal_size();}

            Err(s) =>  {text_line.set_console_msg(s, ConsoleMessageStatus::Error);}
        };
    }

    terminal::disable_raw_mode()?;

    Ok(())
}
