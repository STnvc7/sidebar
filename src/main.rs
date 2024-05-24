mod node;
mod app;
mod viewer;
mod color;
mod file_icon;

use std::io::{stdout, Result};
use std::env;

use crossterm;
use crossterm::{cursor, execute};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};

#[warn(unused_imports)]
fn main() -> Result<()>{
	let args = env::args().nth(1);
	
    execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;
    let _ = app::run(&args);
    execute!(stdout(), cursor::Show, LeaveAlternateScreen)?;

    Ok(())
}
