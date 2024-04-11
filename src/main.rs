mod node;
mod app;
mod text_line;
mod color;

use std::io::stdout;

use crossterm;
use crossterm::{cursor, execute,terminal};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

#[warn(unused_imports)]
fn main(){
    execute!(stdout(), EnterAlternateScreen, cursor::Hide);
    let _ = app::run();
    execute!(stdout(), cursor::Show, LeaveAlternateScreen);
}
