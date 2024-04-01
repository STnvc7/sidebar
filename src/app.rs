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
    

    loop {
        let cmd = read_buffer();
        let result = execute_command(&*cmd, &mut root_node);
        match result{
         Ok(_v) => {
             if _v == 0{
                 break
             }
         }
         Err(_e) => {
             println!("{}", _e);
         }
        }
    }

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


//標準入力からString型で読み込み
fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim().parse().unwrap()
}
//標準入力からのコマンド実行.
fn execute_command(key : &str, tree : &mut node::Node) -> Result<i32, String>{
    match key{
        //quit app
        "q" => Ok(0),
        //open node & show tree
        ""   => {
            &tree.open_node();
            &tree.print_tree(0);
            Ok(1)
        }
         _   => Err(String::from(format!("is invalid command")))
    }
}