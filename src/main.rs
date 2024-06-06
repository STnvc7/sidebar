mod node;
mod app;
mod viewer;
mod color;
mod file_icon;
mod command;
mod error;

use std::io::stdout;
use std::env;

use crossterm;
use crossterm::{cursor, execute};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use anyhow::Result;

#[warn(unused_imports)]
fn main() -> Result<()>{
	let args = env::args().nth(1);

    execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;
    app::run(&args)?;
    execute!(stdout(), cursor::Show, LeaveAlternateScreen)?;

    Ok(())
}
