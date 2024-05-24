use std::collections::VecDeque;
use std::io::{stdout, Result, Write};

use crossterm::{cursor, queue, terminal};
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::{MoveTo, MoveToNextLine};
use crossterm::style::Print;

use crate::color;
use crate::node::NodeType;
use crate::file_icon::get_file_icon;


//表示がスクロールされる際のカーソルの余白(?)説明が下手
const SCROLL_MARGIN : usize = 2;

//表示の際の右の余白
const RIGHT_MARGIN : usize = 2;



//表示するフォルダ/ファイルの情報を保持する構造体
pub struct TextElement{
	pub text : String,
	pub num_lines : usize,
	pub node_type : NodeType, 
	pub is_opened : bool,
	pub rank : usize,
	pub route : VecDeque<usize>,
}

//コンソールメッセージの状態(エラーor通常)
pub enum ConsoleMessageStatus{
	Error,
	Normal,
}

//コンソールメッセージの情報を保持する構造体
struct ConsoleMessage{
	message : String,
	num_lines : usize,
	status  : ConsoleMessageStatus,
}

//コマンドラインに表示される部分
/*
text : 表示するフォルダ・ファイルのTextElement構造体を保持するベクタ．
text_lenght : textの長さ
console_msg : ConsoleMessage
cursor_idx : 現在選択されているtextのインデックス
display_start : ターミナルで表示する最初のtextのインデックス
display_end : ターミナルで表示する最後のインデックス

terminal_width : ターミナルの横幅
terminal_height : ターミナルの縦幅
*/
pub struct Viewer{
	texts : VecDeque<TextElement>,
	console_msg : ConsoleMessage, 
	cursor_idx	  : usize, 
	display_start : usize,
	terminal_width : usize,
	terminal_height : usize,
}

pub fn new() -> Viewer{
	let (width, height) = terminal::size().unwrap();
	return Viewer{
		texts: VecDeque::new(),
		console_msg : ConsoleMessage{message : String::from("to see help,  press 'h'"), num_lines : 1, status: ConsoleMessageStatus::Normal},
		cursor_idx : 0, 
		display_start: 0,
		terminal_width : width as usize,
		terminal_height : height as usize,
	}
}

pub fn get_num_lines(text : &String, left_margin : &usize) -> usize{

	let length = text.len();
	let (terminal_width, _) = terminal::size().unwrap();
	let width = terminal_width as usize - RIGHT_MARGIN - left_margin;
	let num_lines = length.div_ceil(width);

	return num_lines
}

impl Viewer{

	//-----------------------------------------------------------------------------------------
	pub fn set_text(&mut self, text: VecDeque<TextElement>){
		self.texts = text;
		self.update_display_start();
	}

	fn update_display_start(&mut self){
		//区切り線の数
		let separator_size : usize = 2;
		
		//display_startからカーソルのインデックスまでの行数を計算
		let mut num_display_lines : usize = 0;
		for i in self.display_start..=self.cursor_idx{
			num_display_lines += self.texts[i].num_lines;
		}

		//スクロールダウンするときの処理　スクロールして出てくる行の行数に応じてdisplay_startを更新
		if num_display_lines > (self.terminal_height - self.console_msg.num_lines - separator_size - SCROLL_MARGIN){
			self.display_start =  self.display_start + num_display_lines - (self.terminal_height - self.console_msg.num_lines - separator_size - SCROLL_MARGIN);
		}

		//スクロールアップするときの処理
		if self.cursor_idx >= SCROLL_MARGIN  && self.cursor_idx < (self.display_start + SCROLL_MARGIN){
			self.display_start = self.cursor_idx - SCROLL_MARGIN;
		}
	}

	pub fn set_console_msg(&mut self, console_msg: String, status : ConsoleMessageStatus){
		let _num_lines = get_num_lines(&console_msg, &0);
		self.console_msg = ConsoleMessage{ message : console_msg, num_lines : _num_lines, status : status};
	}

	pub fn set_terminal_size(&mut self){
		let (width, height) = terminal::size().unwrap();
		self.terminal_width = width as usize;
		self.terminal_height = height as usize;
	}

	//カーソル操作----------------------------------------------------------------------------------

	pub fn cursor_down(&mut self){

		if self.cursor_idx == (self.texts.len() - 1){
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

	//現在選択されているTextElementから情報を取ってくるやつ達---------------------------------------------
	pub fn get_cursor_route(&self) -> VecDeque<usize>{
		let idx = self.cursor_idx;
		let route = self.texts[idx].route.clone();
		return route
	}

	pub fn get_cursor_node_type(&self) -> NodeType{
		match self.texts[self.cursor_idx].node_type{
			NodeType::Folder => {NodeType::Folder}
			NodeType::File   => {NodeType::File}
		}
	}

	//display用の関数もろもろ-----------------------------------------------------------
	fn get_display_color(&self, idx : &usize, cursor_idx : &usize) -> String{
		//現在選択されているノードの場合はアンダーバーとシアン，それ以外は白
		let color = if idx == cursor_idx { format!("{}{}",color::UNDERLINE,color::CYAN) }
					else { color::WHITE.to_string() };
		return color
	}
	fn get_indent(&self, rank : &usize) -> String{
		return String::from("  ").repeat(*rank)
	}

	fn format_text_for_display(&self, text : &String, num_lines : &usize, indent : &String,
								color : &String, node_type : &NodeType, is_opened : &bool) -> String{

		let icon = get_file_icon(text);

		let line = 
		if *num_lines == 1{ //ファイル名が一行に収まるときの処理
			match node_type{
				NodeType::Folder => {
					let _arrow = if *is_opened {"▼ "} else {"▶ "};		//フォルダ名の先頭に付ける矢印
					format!("{}{}{}{}{}{}", color::RESET,indent, color, _arrow, text, color::RESET) }
		 		NodeType::File   => { format!("{}{}{}{}{}{}", color::RESET, indent, icon, color, text, color::RESET) }
			}
		}

		else{  //ファイル名が一行に収まらない時の処理
			let mut buf : Vec<String> = Vec::new();
			let mut s = String::new();
			for c in text.chars(){
				s.push(c);
				if (indent.len() + s.len()) == (self.terminal_width - RIGHT_MARGIN){
					buf.push(s.clone());
					s.clear();
				}
			}
			buf.push(s.clone());

			let mut _line = String::new();
			_line.push_str(color::RESET);
			match node_type{
				NodeType::Folder => {
					for (i, b) in buf.iter().enumerate(){
						_line.push_str(&format!("{}{}", &indent, &color));

						//フォルダ名の先頭につける矢印
						let _arrow = 
						if i == 0 { if *is_opened{"▼ "} else{"▶ "} }
						else{ "  " };

						_line.push_str(&format!("{}{}{}{}", _arrow, b, color::RESET, &String::from(" ").repeat(RIGHT_MARGIN)));
					}
				}
				NodeType::File => {
					for b in buf.iter(){
						_line.push_str(&format!("{}{}{}{}{}{}", &indent, icon, &color, b, color::RESET, &String::from(" ").repeat(RIGHT_MARGIN)));
					}		
				}
			}
			_line
		};

		return line
	}

	//-----------------------------------------------------------------------------------------
	pub fn display(&self) -> Result<()>{
		
		let separator = String::from("-").repeat(self.terminal_width - 1);

		queue!(stdout(), MoveTo(0,0), Print(format!("{}",color::WHITE)), Print(&separator), MoveToNextLine(1))?;

		let mut i : usize			= self.display_start;
		let mut line_counter :usize = 0;
		loop{

			let _color 		= self.get_display_color(&i, &self.cursor_idx);
			let _indent 	= self.get_indent(&self.texts[i].rank);
			let _num_lines 	= self.texts[i].num_lines;

			//出力の行数分だけターミナルを掃除
			queue!(stdout(), cursor::SavePosition)?;
			for _ in 0.._num_lines{
				queue!(stdout(), Clear(ClearType::CurrentLine), MoveToNextLine(1))?;
			}
			queue!(stdout(), cursor::RestorePosition)?;

			//表示用に整形
			let _line = self.format_text_for_display(&self.texts[i].text, &self.texts[i].num_lines, &_indent, &_color, 
													 &self.texts[i].node_type, &self.texts[i].is_opened);
			queue!(stdout(), Print(_line), MoveToNextLine(1))?;

			//ループ終了条件の処理
			i += 1;
			line_counter += _num_lines;
			if i > (self.texts.len() - 1) || line_counter > (self.terminal_height - &self.console_msg.num_lines - 3){
				break
			}
		}
		//---------------------------------------------------------------------------------------------------------------------------

		
		queue!(stdout(), Clear(ClearType::FromCursorDown), MoveTo(0, (self.terminal_height-&self.console_msg.num_lines-1) as u16),
			   Print(format!("{}{}",color::WHITE, &separator)), MoveToNextLine(1))?;
		let console_msg = match self.console_msg.status{
			ConsoleMessageStatus::Normal => { format!("{}{}", color::WHITE, self.console_msg.message) }
			ConsoleMessageStatus::Error  => { format!("{}{}", color::RED, self.console_msg.message)}
		};
		queue!(stdout(), Print(console_msg))?;

		stdout().flush()?;
		
		Ok(())
	}
}
