use std::collections::VecDeque;
use std::io::stdout;
use crossterm::{cursor, execute, terminal};

pub struct{
	
}

pub struct TextLine{
	text : VecDeque<String>,
	console_msg : String, 
	cursor_idx	  : usize, 
	display_start : usize,
	display_end   : usize,
}

pub fn new() -> TextLine{
	let (_, bottom) = terminal::size().unwrap();
	return TextLine{
		text: VecDeque::new(),
		console_msg : String::new(),
		cursor_idx : 0, 
		display_start: 0,
		display_end : 0,
	}
}

impl TextLine{

	pub fn set_text(&mut self, text: VecDeque<String>){
		self.display_end = text.len() - 1;
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

	pub fn cursor_down(&mut self){
		if self.cursor_idx == self.display_end{
			return
		}

		self.cursor_idx += 1;
	}

	pub fn get_cursor_route(&self){

	}

	pub fn display(&self){
		execute!(stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0,0));
		execute!(stdout(), cursor::MoveTo(0,0));

		for i in self.display_start..self.display_end{
			let output_color = if i == self.cursor_idx { color::GREEN } else { color::WHITE };
			print!("{}{}", output_color, self.text[i]);
			execute!(stdout(), cursor::MoveToNextLine(1));
		}
	}
}