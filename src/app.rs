use std::env;
use std::io;
use std::io::{stdout, Write};
use crossterm::{
    cursor, execute, ExecutableCommand, terminal,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    event::{read, Event, KeyCode, KeyEvent},
};

use crate::node;

//無限ループをキー入力を待ち続ける

pub fn run() -> io::Result<()>{

	let root = env::current_dir().unwrap();
    let mut root_node = node::new_node(root).unwrap();
    let selected_node = &mut root_node;

    terminal::enable_raw_mode()?;
    loop{
        root_node.print_tree(0);


        // イベントの取得
        let event = read()?;

        let result = match event {
            Event::Key(e) => {execute_command_from_key_event(e, &mut root_node)},
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

fn execute_command_from_key_event(key : KeyEvent, tree : &mut node::Node) -> Result<i32, String>{

    match key.code{
        //quit app
        KeyCode::Char('q') => Ok(0),

        //open or close node
        KeyCode::Enter   => {
            execute!(stdout(), terminal::Clear(terminal::ClearType::All));

            if tree.is_opened() == false{
                &tree.open_node();
            }
            else{
                &tree.close_node();
            }

            Ok(1)
        }

        KeyCode::Down => {
            Ok(1)
        }

        KeyCode::Char(c)   => Err(String::from(format!("{} is invalid command", c))),
        _ => Err(String::from("no covered key"))
    }
}