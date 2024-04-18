use std::collections::VecDeque;
use std::io::{stdout, Result, Write};

use crossterm::{cursor, execute, queue, terminal};
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::{MoveTo, MoveToNextLine};
use crossterm::style::Print;
use std::path::PathBuf;


use crate::color;
use crate::node::NodeType;

const CONSOLE_MSG_NUM_LINE : usize = 3;
const SCROLL_MARGIN : usize = 2;

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
	display_end   : usize,

	terminal_width : usize,
	terminal_height : usize,
}

pub fn new() -> TextLine{
	let (width, height) = terminal::size().unwrap();
	return TextLine{
		text: VecDeque::new(),
		text_length : 0,
		console_msg : ConsoleMessage{message : String::new(), status: ConsoleMessageStatus::Error},
		cursor_idx : 0, 
		display_start: 0,
		display_end : 0,
		terminal_width : width as usize,
		terminal_height : height as usize,
	}
}

impl TextLine{

	//-----------------------------------------------------------------------------------------
	pub fn set_text(&mut self, text: VecDeque<TextElement>){
		self.text_length = text.len();

		//表示する文字列の行数長がターミナルの高さよりも小さい時
		if (self.text_length - self.display_start) < self.terminal_height - CONSOLE_MSG_NUM_LINE{
			self.display_end = self.text_length - 1; 

			if self.cursor_idx >= SCROLL_MARGIN  && self.cursor_idx < (self.display_start + SCROLL_MARGIN) {
				self.display_start = self.cursor_idx - SCROLL_MARGIN;
			}

		}

		//表示する文字列の行数がターミナルの高さより大きい時
		else{
			self.display_end = self.display_start + (self.terminal_height - CONSOLE_MSG_NUM_LINE) - 1;

			if self.cursor_idx + SCROLL_MARGIN < self.text_length  && self.cursor_idx > (self.display_end - SCROLL_MARGIN){
				self.display_start = (self.cursor_idx + 1 + SCROLL_MARGIN) - (self.terminal_height - CONSOLE_MSG_NUM_LINE);
				self.display_end = self.cursor_idx + SCROLL_MARGIN;
			}

			if self.cursor_idx >= SCROLL_MARGIN  && self.cursor_idx < (self.display_start + SCROLL_MARGIN) {
				self.display_start = self.cursor_idx - SCROLL_MARGIN;
				self.display_end = (self.cursor_idx - SCROLL_MARGIN) + self.terminal_height - CONSOLE_MSG_NUM_LINE;
			}
		}

		self.text = text;
	}

	pub fn set_console_msg(&mut self, console_msg: String, status : ConsoleMessageStatus){
		self.console_msg = ConsoleMessage{ message : console_msg, status : status};
	}

	pub fn set_terminal_size(&mut self){
		let (width, height) = terminal::size().unwrap();
		self.terminal_width = width as usize;
		self.terminal_height = height as usize;
	}

	//-----------------------------------------------------------------------------------------

	pub fn cursor_down(&mut self){

		if self.cursor_idx == (self.text_length - 1){
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
		
		let separator = String::from("-").repeat(self.terminal_width - 1);

		queue!(stdout(), MoveTo(0,0), Print(format!("{}",color::WHITE)))?;
		queue!(stdout(), Print(&separator), MoveToNextLine(1))?;

		for i in self.display_start..=self.display_end {

			let _text = &self.text[i].text;
			let _rank = self.text[i].rank;

			let _color = if i == self.cursor_idx { format!("{}{}",color::UNDERLINE,color::GREEN) } else { color::WHITE.to_string() };
			let _indent = String::from("  ").repeat(_rank);

			match self.text[i].node_type{
				NodeType::Folder => { queue!(stdout(), Clear(ClearType::CurrentLine), Print(format!("{}{}{}❯ {}{}", color::RESET,_indent, _color, _text, color::RESET)))?;}
				NodeType::File   => { queue!(stdout(), Clear(ClearType::CurrentLine), Print(format!("{}{}{}{}{}", color::RESET, _indent, _color, _text, color::RESET)))?;}
			}
			queue!(stdout(), cursor::MoveToNextLine(1))?;
		}

		queue!(stdout(), MoveTo(0, (self.terminal_height as u16) - 2), Print(format!("{}{}",color::WHITE, &separator)), MoveToNextLine(1))?;
		let console_msg = match self.console_msg.status{
			ConsoleMessageStatus::Normal => { format!("{}{}", color::WHITE, self.console_msg.message) }
			ConsoleMessageStatus::Error  => { format!("{}{}", color::RED, self.console_msg.message)}
		};
		queue!(stdout(), Clear(ClearType::CurrentLine), Print(console_msg))?;
		stdout().flush()?;
		
		Ok(())
	}
}