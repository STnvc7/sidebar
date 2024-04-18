use std::env;
use std::io;
use std::process::Command;

use crossterm::terminal;
use crossterm::event::{read, Event, KeyCode};

use crate::node;
use crate::node::NodeType;
use crate::text_line::ConsoleMessageStatus;
use crate::text_line;

enum Commands{
    Quit,
    Up,
    Down,
    ShowPath,
    OpenFile,
    OpenFolder,
    Resize,
}

//無限ループでキー入力を待ち続ける
pub fn run() -> io::Result<()>{

    let root = env::current_dir().unwrap();

    let mut tree = node::build_tree(&root);
    let mut text_line = text_line::new();

    let _text = tree.format();
    text_line.set_text(_text);
    text_line.display()?;

    terminal::enable_raw_mode()?;
    loop{
        // イベントの取得
        let event = read()?;

        let command = match event {
            //キーイベント
            Event::Key(e) => {
                match e.code{
                    //quit app
                    KeyCode::Char('q')  =>  Ok(Commands::Quit),
                    KeyCode::Char('p')  =>  Ok(Commands::ShowPath),
                    KeyCode::Enter      => {
                        match text_line.get_cursor_node_type(){
                            NodeType::Folder => Ok(Commands::OpenFolder),
                            NodeType::File   => Ok(Commands::OpenFile),
                        }
                    }
                    KeyCode::Down       =>  Ok(Commands::Down),
                    KeyCode::Up         =>  Ok(Commands::Up),

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

            Ok(Commands::ShowPath)   => {let _path = text_line.get_cursor_path().to_string_lossy().into_owned();
                                         text_line.set_console_msg(_path, ConsoleMessageStatus::Normal);}

            Ok(Commands::OpenFolder) => {let route = text_line.get_cursor_route();
                                         tree.open_node(route.clone());}

            Ok(Commands::OpenFile)   => {let _path = text_line.get_cursor_path();
                                         let _ = Command::new("rsubl").arg(_path).spawn();} //TODO!!!!!!!!!!!!!!!

            Ok(Commands::Resize)     => {text_line.set_terminal_size();}

            Err(s) =>  {text_line.set_console_msg(s, ConsoleMessageStatus::Error);}
        };

        let _text = tree.format();
        text_line.set_text(_text);
        text_line.display()?;
    }

    terminal::disable_raw_mode()?;

    Ok(())
}
