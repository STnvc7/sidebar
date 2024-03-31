use std::env;
use std::io;
use std::io::stdout;
use crossterm::{
    cursor, execute, ExecutableCommand, terminal,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    event::{read, Event, KeyCode, KeyEvent},
};

use crate::node;
use crate::cursor_control;

//無限ループをキー入力を待ち続ける

pub fn run() -> io::Result<()>{

	let root = env::current_dir().unwrap();
    let mut root_node = node::new_node(root).unwrap();
    root_node.print_tree(0);
    
    terminal::enable_raw_mode()?;
    loop{
        // イベントの取得
        let event = read()?;

        let result = match event {
            Event::Key(e) => {execute_command_from_key_event(e, &mut root_node)},
            _ => {Err(String::from("cannot accept keys..."))}
        };

        cursor_control::to_edge_cursor();

        match result{
         Ok(_v) => {
             if _v == 0{
                 cursor_control::to_edge_cursor();
                 break
             }
         }
         Err(_e) => {
             println!("{}", _e);
         }
        }
    }
    terminal::disable_raw_mode()?;
    execute!(stdout(), terminal::Clear(terminal::ClearType::All));
    cursor_control::to_top();
    Ok(())
}

fn execute_command_from_key_event(key : KeyEvent, tree : &mut node::Node) -> Result<i32, String>{
    match key.code{
        //quit app
        KeyCode::Char('q') => Ok(0),
        //open node & show tree
        KeyCode::Enter   => {
            &tree.open_node();
            &tree.print_tree(0);
            Ok(1)
        }
         _   => Err(String::from(format!("is invalid command")))
    }
}


// //標準入力からString型で読み込み　※未使用
// fn read_buffer() -> String {
//     let mut buffer = String::new();
//     io::stdin().read_line(&mut buffer).expect("Failed to read line.");
//     buffer.trim().parse().unwrap()
// }
// //標準入力からのコマンド実行. ※未使用
// fn execute_command(key : &str, tree : &mut node::Node) -> Result<i32, String>{
//     match key{
//         //quit app
//         "q" => Ok(0),
//         //open node & show tree
//         ""   => {
//             &tree.open_node();
//             &tree.print_tree(0);
//             Ok(1)
//         }
//          _   => Err(String::from(format!("is invalid command")))
//     }
// }