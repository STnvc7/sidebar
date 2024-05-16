mod node;
mod app;
mod viewer;
mod color;
mod file_icon;

use std::io::{stdout, Result};

use crossterm;
use crossterm::{cursor, execute};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};

#[warn(unused_imports)]
fn main() -> Result<()>{

    execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;
    let _ = app::run();
    execute!(stdout(), cursor::Show, LeaveAlternateScreen)?;

    Ok(())
}
