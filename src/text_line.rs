use std::collections::VecDeque;
use std::io::stdout;
use crossterm::{cursor, execute, terminal};

pub struct TextLine{
	text : VecDeque<String>,
	console_msg : String, 
	display_start : usize,
	display_end   : usize,
}

pub fn new() -> TextLine{
	let (_, bottom) = terminal::size().unwrap();
	return TextLine{
		text: VecDeque::new(),
		console_msg : String::new(),
		display_start: 0,
		display_end : 0,
	}
}

impl TextLine{

	pub fn set_text(&mut self, text: VecDeque<String>){
		self.display_end = text.len();
		self.text = text;
	}

	pub fn set_console_msg(&mut self, console_msg: String){
		self.console_msg = console_msg;
	}

	pub fn set_display_start(&mut self, value: usize){
		self.display_start = value;
	}

	pub fn set_display_end(&mut self, value: usize){
		self.display_end = value;
	}

	pub fn display_text(&self){
		execute!(stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0,0));
		execute!(stdout(), cursor::MoveTo(0,0));

		for i in self.display_start..self.display_end{
			print!("{}", self.text[i]);
			execute!(stdout(), cursor::MoveToNextLine(1));
		}
	}
}