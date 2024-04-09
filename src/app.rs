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

    // route.push_back(5);
    // route.push_back(2);
    tree.set_selected(true);
    // tree.set_opened(true);
    // tree.route_node(route.clone());
    // text_buffer = tree.convert_to_string_vec(0);

    // for text in text_buffer.iter(){
    //     println!("{}", text);
    // }

    // return Ok(());

    terminal::enable_raw_mode()?;
    loop{
        tree.route_node(route.clone());
        let text = tree.convert_to_string_vec(0);
        text_line.set_text(text);
        text_line.display_text();

        
        // イベントの取得
        let event = read()?;

        let result = match event {
            Event::Key(e) => {execute_command_from_key_event(e, &mut tree)},
            _ => {Err(String::from("cannot accept keys..."))}
        };

        match result{
            Ok(_v) => {
                if _v == 0{
                    break
                }
            }
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

fn execute_command_from_key_event(key : KeyEvent, tree : &mut Box<node::Node>) -> Result<i32, String>{

    match key.code{
        //quit app
        KeyCode::Char('q') => Ok(0),

        //open or close node
        KeyCode::Enter   => {
            execute!(stdout(), terminal::Clear(terminal::ClearType::All));
            &tree.open_node();

            Ok(1)
        }

        KeyCode::Down => {
            //&tree.select_down();
            Ok(1)
        }

        KeyCode::Char(c)   => Err(String::from(format!("{} is invalid command", c))),
        _ => Err(String::from("no covered key"))
    }
}