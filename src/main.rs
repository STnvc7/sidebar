mod node;
mod app;
mod text_line;
mod color;

use std::io::stdout;

use crossterm::{cursor, execute,};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

#[warn(unused_imports)]
fn main(){
    execute!(stdout(), EnterAlternateScreen, cursor::Hide, Clear(ClearType::All));
    app::run();
    execute!(stdout(), cursor::Show, LeaveAlternateScreen);
}
