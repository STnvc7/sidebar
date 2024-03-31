use crossterm::{cursor, execute};
use std::io::stdout;

pub fn to_edge_cursor(){
    let cur_pos = cursor::position().unwrap();
    execute!(stdout(), cursor::Hide, cursor::MoveTo(0,cur_pos.1));
}

pub fn to_top(){
    execute!(stdout(), cursor::Hide, cursor::MoveTo(0,0));
}