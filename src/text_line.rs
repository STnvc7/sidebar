use std::collections::VecDeque;
use std::io::{stdout, Result};
use crossterm::{cursor, execute, terminal};
use crossterm::style::Print;
use std::path::PathBuf;


use crate::color;
use crate::node::NodeType;

pub struct TextElement{
	pub text : String,
	pub path : PathBuf,
	pub node_type : NodeType, 
	pub rank : usize,
	pub route : VecDeque<usize>,
}

pub enum ConsoleMessageStatus{
	Error,
	Normal,
}

struct ConsoleMessage{
	message : String,
	status  : ConsoleMessageStatus,
}

pub struct TextLine{
	text : VecDeque<TextElement>,
	text_length : usize,
	console_msg : ConsoleMessage, 

	cursor_idx	  : usize, 

	display_start : usize,
	terminal_size : usize,
}

pub fn new() -> TextLine{
	let (_, bottom) = terminal::size().unwrap();
	return TextLine{
		text: VecDeque::new(),
		text_length : 0,
		console_msg : ConsoleMessage{message : String::new(), status: ConsoleMessageStatus::Error},
		cursor_idx : 0, 
		display_start: 0,
		terminal_size : bottom as usize,
	}
}

impl TextLine{

	//-----------------------------------------------------------------------------------------
	pub fn set_text(&mut self, text: VecDeque<TextElement>){
		self.text_length = text.len();
		self.text = text;
	}

	pub fn set_console_msg(&mut self, console_msg: String, status : ConsoleMessageStatus){
		self.console_msg = ConsoleMessage{ message : console_msg, status : status};
	}

	//-----------------------------------------------------------------------------------------
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
	//-----------------------------------------------------------------------------------------
	pub fn get_cursor_path(&self) -> PathBuf {
		let _path = self.text[self.cursor_idx].path.clone();
		return _path
	}

	pub fn get_cursor_route(&self) -> VecDeque<usize>{
		let idx = self.cursor_idx;
		let route = self.text[idx].route.clone();
		return route
	}

	pub fn get_cursor_node_type(&self) -> NodeType{
		match self.text[self.cursor_idx].node_type{
			NodeType::Folder => {NodeType::Folder}
			NodeType::File   => {NodeType::File}
		}
	}
	//-----------------------------------------------------------------------------------------
	pub fn display(&self) -> Result<()>{
		
		execute!(stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0,0), Print(format!("{}", color::WHITE)))?;

		let start = self.display_start;
		let end   = if self.text_length < self.terminal_size { self. text_length }
		    		else { self.display_start + self.terminal_size };

		for i in start..end {

			let _text = &self.text[i].text;
			let _rank = self.text[i].rank;

			let _color = if i == self.cursor_idx { color::GREEN } else { color::WHITE };
			let _indent = String::from("  ").repeat(_rank);

			match self.text[i].node_type{
				NodeType::Folder => { execute!(stdout(), Print(format!("{}{}>{}", _indent, _color, _text)))?;}
				NodeType::File   => { execute!(stdout(), Print(format!("{}{}{}", _indent, _color, _text)))?;}
			}
			execute!(stdout(), cursor::MoveToNextLine(1))?;
		}

		let console_msg = match self.console_msg.status{
			ConsoleMessageStatus::Normal => { format!("{}{}", color::WHITE, self.console_msg.message) }
			ConsoleMessageStatus::Error  => { format!("{}{}", color::RED, self.console_msg.message)}
		};
		execute!(stdout(), cursor::MoveTo(0, self.terminal_size as u16), Print(console_msg))?;

		Ok(())
	}
}