use std::env;
use std::io;
use std::collections::VecDeque;
use std::process::Command;

use crossterm::terminal;
use crossterm::event::{read, Event, KeyCode};

use crate::node;
use crate::node::NodeType;
use crate::text_line::ConsoleMessageStatus;
use crate::text_line;

//無限ループでキー入力を待ち続ける

pub fn run() -> io::Result<()>{

    let root = env::current_dir().unwrap();

    let mut tree = node::build_tree(&root);
    let mut text_line = text_line::new();

    let _text = tree.format_for_textline(0, VecDeque::new());
    text_line.set_text(_text);
    text_line.display()?;

    terminal::enable_raw_mode()?;
    loop{
        // イベントの取得
        let event = read()?;

        let result = match event {
            //キーイベント
            Event::Key(e) => {
                match e.code{
                    //quit app
                    KeyCode::Char('q')  => { break; }
                    KeyCode::Char('p')  => { let _path = text_line.get_cursor_path();
                                             Ok(_path.to_string_lossy().into_owned())}

                    //open or close node
                    KeyCode::Enter      => {
                        match text_line.get_cursor_node_type(){

                            //フォルダの時はフォルダを開く又は閉じる
                            NodeType::Folder => { let route = text_line.get_cursor_route();
                                                  tree.open_node(route.clone());
                                                }
                            //ファイルの時はファイルを開く
                            NodeType::File   => { let _path = text_line.get_cursor_path();
                                                  Command::new("subl").arg(_path).spawn(); //TODO!!!!!!!!!!!!!!!
                                                }
                        }
                        Ok(String::new())
                    }

                    KeyCode::Down       => { text_line.cursor_down(); Ok(String::new()) }
                    KeyCode::Up         => { text_line.cursor_up();   Ok(String::new()) }

                    KeyCode::Char(c)    => Err(String::from(format!("{} is invalid command", c))),

                    _ => Err(String::from("no covered key"))
                }        
            },
            _ => {Err(String::from("cannot accept keys..."))}
        };

        match result{
            Ok(s)   => {text_line.set_console_msg(s, ConsoleMessageStatus::Normal);}
            Err(s) =>  {text_line.set_console_msg(s, ConsoleMessageStatus::Error);}
        };

        let _text = tree.format_for_textline(0, VecDeque::new());
        text_line.set_text(_text);
        
        text_line.display()?;
    }

    terminal::disable_raw_mode()?;

    Ok(())
}