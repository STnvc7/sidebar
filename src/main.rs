mod node;
mod app;
mod cursor_control;

use std::io::{stdout, Result};

use crossterm::{ cursor, execute,terminal};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

#[warn(unused_imports)]
fn main(){
    execute!(stdout(), Clear(terminal::ClearType::All));
    //execute!(stdout(), EnterAlternateScreen);
    app::run();
    //execute!(stdout(), LeaveAlternateScreen);
}
