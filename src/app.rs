use std::env;
use std::io;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::collections::VecDeque;

use crossterm::{
    cursor, execute, ExecutableCommand, terminal,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    event::{read, Event, KeyCode, KeyEvent},
};

use crate::node;
use crate::text_line;

//無限ループでキー入力を待ち続ける

pub fn run() -> io::Result<()>{

    let root = env::current_dir().unwrap();

    let mut tree = node::build_tree(&root);
    let mut route : VecDeque<usize> = VecDeque::new();
    let mut text_line = text_line::new();

    tree.set_selected(true);
    text_line.set_text(tree.convert_to_string_vec(0));
    text_line.display_text();

    terminal::enable_raw_mode()?;
    loop{
        // イベントの取得
        let event = read()?;

        let result = match event {

            //キーイベント
            Event::Key(e) => {
                match e.code{
                    //quit app
                    KeyCode::Char('q')  => { break;}

                    //open or close node
                    KeyCode::Enter      => { tree.open_node(); Ok(())}

                    //
                    KeyCode::Down       => {
                                            let result = tree.get_route(route.clone(), "down");
                                            match result{
                                                Some(v) => {route = v;
                                                            tree.set_selected_all(false);
                                                            tree.set_route(route.clone());}
                                                None    => {}
                                            }
                                            Ok(())}

                    KeyCode::Up       => {
                                            let result = tree.get_route(route.clone(), "up");
                                            match result{
                                                Some(v) => {route = v;
                                                            tree.set_selected_all(false);
                                                            tree.set_route(route.clone());}
                                                None    => {tree.set_selected_all(false);
                                                            tree.set_selected(true);}
                                            }
                                            Ok(())}

                    KeyCode::Char(c)    => Err(String::from(format!("{} is invalid command", c))),

                    _ => Err(String::from("no covered key"))
                }        
            },
            _ => {Err(String::from("cannot accept keys..."))}
        };

        let _text = tree.convert_to_string_vec(0);
        text_line.set_text(_text);
        text_line.display_text();

        match result{
            Ok(v)   => {}
            Err(_e) => {
                execute!(stdout(), terminal::Clear(terminal::ClearType::CurrentLine));
                print!("{}{}", "\x1b[31m", _e);
                execute!(stdout(), cursor::MoveToNextLine(1));
            }
        }
    }

    terminal::disable_raw_mode()?;

    Ok(())
}