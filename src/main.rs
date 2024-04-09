mod node;
mod app;
mod text_line;

use std::io::stdout;

use crossterm::{cursor, execute,};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

#[warn(unused_imports)]
fn main(){
    execute!(stdout(), cursor::Hide, Clear(ClearType::All));
    //execute!(stdout(), EnterAlternateScreen);
    app::run();
    //execute!(stdout(), LeaveAlternateScreen);
    execute!(stdout(), cursor::Show);
}
