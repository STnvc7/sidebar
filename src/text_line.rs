use std::collections::VecDeque;
use std::io::{stdout, Result};
use crossterm::{cursor, execute, terminal};
use crossterm::style::Print;

use crate::color;
use crate::node::NodeType;

pub struct TextElement{
	pub text : String,
	pub node_type : NodeType, 
	pub rank : usize,
	pub route : VecDeque<usize>,
}

pub struct TextLine{
	text : VecDeque<TextElement>,
	text_length : usize,
	console_msg : String, 

	cursor_idx	  : usize, 

	display_start : usize,
	terminal_size : usize,
}

pub fn new() -> TextLine{
	let (_, bottom) = terminal::size().unwrap();
	return TextLine{
		text: VecDeque::new(),
		text_length : 0,
		console_msg : String::new(),
		cursor_idx : 0, 
		display_start: 0,
		terminal_size : bottom as usize,
	}
}

impl TextLine{

	pub fn set_text(&mut self, text: VecDeque<TextElement>){
		self.text_length = text.len();
		self.text = text;
	}

	pub fn set_console_msg(&mut self, console_msg: String){
		self.console_msg = console_msg;
	}

	pub fn set_display_start(&mut self, value: usize){
		self.display_start = value;
	}

	pub fn set_terminal_size(&mut self, value: usize){
		self.terminal_size = value;
	}

	pub fn cursor_down(&mut self){
		if self.cursor_idx == self.text_length - 1{
			return
		}

		self.cursor_idx += 1;
	}
	
	pub fn cursor_up(&mut self){
		if self.cursor_idx == 0{
			return
		}

		self.cursor_idx -= 1;
	}

	pub fn get_cursor_route(&self) -> VecDeque<usize>{
		let idx = self.cursor_idx;
		let route = self.text[idx].route.clone();
		return route
	}

	pub fn display(&self) -> Result<()>{
		execute!(stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0,0));
		print!("{}", color::WHITE);

		let start = self.display_start;
		let end   = if self.text_length < self.terminal_size { self. text_length }
		    		else { self.display_start + self.terminal_size };

		for i in start..end {

			let _text = &self.text[i].text;
			let _rank = self.text[i].rank;

			let _color = if i == self.cursor_idx { color::GREEN } else { color::WHITE };
			let _indent = String::from("  ").repeat(_rank);

			match self.text[i].node_type{
				NodeType::Folder => { execute!(stdout(), Print(format!("{}{}>{}", _indent, _color, _text)));}
				NodeType::File   => { execute!(stdout(), Print(format!("{}{}{}", _indent, _color, _text)));}
			}
			execute!(stdout(), cursor::MoveToNextLine(1))?;
		}

		execute!(stdout(), cursor::MoveTo(0, self.terminal_size as u16), 
				 Print(format!("{}{}", color::RED, self.console_msg)))?;

		Ok(())
	}
}