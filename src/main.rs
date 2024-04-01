mod node;
mod app;

use std::io::{stdout, Result};

use crossterm::{ cursor, execute,terminal};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

#[warn(unused_imports)]
fn main(){
    execute!(stdout(), cursor::Hide, Clear(terminal::ClearType::All));
    app::run();
    execute!(stdout(), cursor::Show);
}
