use crate::color as COLOR;
use crate::icon;
use crate::node::NodeType;
use crate::node_map::NodeMap;
use crate::config::Config;

use anyhow::Result;
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crossterm::cursor::{MoveTo, MoveToNextLine, MoveDown};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{queue, terminal};

#[derive(Debug, Clone)]
pub enum ConsoleMessageStatus{
    Info,
    Error,
    Notify,
}
#[derive(Debug)]
struct ConsoleMessage{
    pub message: String,
    pub status: ConsoleMessageStatus,
}
impl ConsoleMessage{
    fn new(message: String, status: ConsoleMessageStatus) -> ConsoleMessage{
        ConsoleMessage{
            message: message,
            status: status
        }
    }
    fn get_num_lines(&self, terminal_width: usize) -> usize {
        return self.message.len().div_ceil(terminal_width);
    }

}



#[allow(dead_code)]
pub struct Viewer {
    node_map: Arc<Mutex<NodeMap>>,
    id_list: Vec<Uuid>,
    console_message: Option<ConsoleMessage>,
    display_start_idx: usize,
    display_end_idx: usize,
    cursor_idx: usize,
    secondoy_cursor_mode: bool,
    terminal_width: usize,
    terminal_height: usize,
    config: Arc<Config>,
}

#[allow(dead_code)]
impl Viewer {
    // ----------------------------------------------------------------
    // コンストラクタ
    // ----------------------------------------------------------------
    pub fn new(node_map: Arc<Mutex<NodeMap>>, config: Arc<Config>) -> Viewer {
        let (width, height) = terminal::size().unwrap();
        let id_list: Vec<Uuid> = Vec::new();

        Viewer {
            node_map: node_map,
            id_list: id_list,
            console_message: None,
            display_start_idx: 0,
            display_end_idx: 1,
            cursor_idx: 0,
            secondoy_cursor_mode: false,
            terminal_width: width as usize,
            terminal_height: height as usize,
            config: config,
        }
    }

    // カーソルが選択しているノードのIDを取得
    pub fn get_cursor_id(&self) -> Uuid {
        self.id_list[self.cursor_idx].clone()
    }

    // コンソールメッセージを保存
    pub fn set_console_message(&mut self, message: String, status: ConsoleMessageStatus) {
        let prefix = String::from("> ");
        let msg = format!("{}{}", prefix, message);
        self.console_message = Some(ConsoleMessage::new(msg, status));
    }
    pub fn clear_console_message(&mut self) {
        self.console_message = None
    }

    // ターミナルのサイズが変更されたときに呼び出される
    pub fn resize(&mut self) -> Result<()>{
        let (width, height) = terminal::size()?;
        self.terminal_width = width as usize;
        self.terminal_height = height as usize;
        Ok(())
    }

    pub fn activate_secondly_cursor(&mut self) {
        self.secondoy_cursor_mode = true;
    }
    pub fn deactivate_secondly_cursor(&mut self) {
        self.secondoy_cursor_mode = false;
    }

    // カーソルを上に -----------------------
    pub fn cursor_up(&mut self) {
        // 更新------------------
        if self.cursor_idx == 0 {
            return;
        }
        self.cursor_idx -= 1;

        return;
    }

    // カーソルを下に -----------------------
    pub fn cursor_down(&mut self) {
        let length = self.id_list.len();

        // 更新------------------
        if self.cursor_idx >= length-1 {
            return;
        }
        self.cursor_idx += 1;

        return;
    }

    // ランクの異なるノードまでカーソルを上向きにジャンプ ---------------------
    pub fn cursor_jump_up(&mut self) -> Result<()> {
        let node_map = self.node_map.lock().unwrap();
        let mut current_idx = self.cursor_idx;
        let current_rank = node_map.get_rank(&self.id_list[current_idx])?;

        // ランクの異なるノードがでてくるまでループ
        loop {
            // 上限
            if current_idx <= 0 {
                break;
            } else {
                current_idx = current_idx - 1
            }
            let next_rank = node_map.get_rank(&self.id_list[current_idx])?;
            
            //カーソルを更新して終了
            if next_rank != current_rank {
                self.cursor_idx = current_idx;
                break;
            }
        }
        return Ok(());
    }

    // ランクの異なるノードまでカーソルを下向きにジャンプ ---------------------
    pub fn cursor_jump_down(&mut self) -> Result<()> {
        let node_map = self.node_map.lock().unwrap();

        let mut current_idx = self.cursor_idx;
        let current_rank = node_map.get_rank(&self.id_list[current_idx])?;
        
        loop {
            // 下限
            if current_idx >= self.id_list.len() - 1 {
                break;
            } else {
                current_idx = current_idx + 1
            }
            let next_rank = node_map.get_rank(&self.id_list[current_idx])?;

            // カーソルを更新して終了
            if next_rank != current_rank {
                self.cursor_idx = current_idx;
                break;
            }
        }
        return Ok(());
    }

    // node_mapと同期 -------------------------
    pub fn sync(&mut self) -> Result<()> {
        let node_map = self.node_map.lock().unwrap();

        // id_listの更新
        let id_list = node_map.serialize()?;
        self.id_list = id_list;

        // カーソルを長さに合わせる
        if self.id_list.len() <= self.cursor_idx {
            self.cursor_idx = self.id_list.len() - 1;
        }

        Ok(())
    }

    // 各行の出力を生成 ----------------------------------
    fn format(&self, name: String, icon: String, rank: usize, color: &str) -> String {
        let indent = String::from("  ").repeat(rank);
        let prefix_length = icon.len() + indent.len();

        // ターミナルのサイズに合わせる ------
        let modified_name = if self.terminal_width < (prefix_length + name.len()) {
            let available_length = self.terminal_width - 1;
            let mut _name = String::new();
            for c in name.chars() {
                if prefix_length + _name.len() >= available_length {
                    _name.push_str("…");
                    break;
                }
                _name.push(c);
            }
            _name
        } else {
            name
        };

        // 結合 ----------------
        let line = format!("{}{} {}{}", indent, icon, color, modified_name);
        return line;
    }

    // カーソル上のノード => 青
    // セカンダリーカーソル上のノード => 緑
    fn get_line_color(&self, i: usize) -> &str {
        if i != self.cursor_idx {
            return COLOR::RESET
        }

        if self.secondoy_cursor_mode{
            return COLOR::front::GREEN
        }

        return COLOR::front::BLUE
    }

    // 表示開始位置の更新
    fn update_display_size(&mut self) {
        let mut display_height = self.terminal_height;

        // コンソールメッセージがある際はメッセージの行数分表示の範囲を狭める
        if let Some(ref console_msg) = self.console_message {
            let num_lines = console_msg.get_num_lines(self.terminal_width);
            display_height -= num_lines;

        }
        
        // display_startの更新---------------------------------------
        if self.cursor_idx >= self.display_start_idx + display_height - 1 {
            // カーソルが表示範囲を超えたとき
            // 先に1を足しておかないとusizeが一瞬負の値になってパニックする
            self.display_start_idx = (self.cursor_idx + 1) - display_height;
        }
        else if self.cursor_idx < self.display_start_idx{
            self.display_start_idx = self.cursor_idx;
        }

        // display_endの更新 ---------------------------------------
        if self.display_start_idx + display_height > self.id_list.len() {
            self.display_end_idx = self.id_list.len() - 1;
        }
        else {
            self.display_end_idx = self.display_start_idx + display_height - 1;
        }
    }

    // 表示をおこなうメソッド -----------------------------------------
    pub fn display(&mut self) -> Result<()> {
        self.update_display_size();

        let node_map = self.node_map.lock().unwrap();
        queue!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;

        // ノードの表示
        for i in self.display_start_idx..=self.display_end_idx {
            
            let id = self.id_list[i];
            
            let name = node_map.get_name(&id)?;
            let rank = node_map.get_rank(&id)?;
            let node_type = node_map.get_node_type(&id)?;
            let icon = match node_type {
                NodeType::Folder => {
                    let is_open = node_map.get_is_open(&id)?;
                    icon::get_folder_icon(is_open, self.config.nerd_font)
                }
                NodeType::File => {
                    icon::get_file_icon(&name, self.config.nerd_font)
                }
                NodeType::Unknown => {
                    String::from("?")
                }
            };
            let color = self.get_line_color(i);

            let line = self.format(name, icon, rank, color);
            let out = format!("{}{}{}", COLOR::RESET, line, COLOR::RESET);

            queue!(stdout(), Print(out), MoveToNextLine(1))?;
        }

        // コンソールメッセージ ---------------------------------------------
        if let Some(ref console_msg) = self.console_message {
            let message = console_msg.message.clone();
            let status = console_msg.status.clone();

            // 綺麗に表示する用
            let num_line = console_msg.get_num_lines(self.terminal_width);
            let blank = if message.len().rem_euclid(self.terminal_width) != 0 {
                String::from(" ").repeat(
                    self.terminal_width - message.len().rem_euclid(self.terminal_width)
                )
            } else {
                String::from("")
            };
            let color = match status{
                ConsoleMessageStatus::Info => COLOR::back::BLUE,
                ConsoleMessageStatus::Error => COLOR::back::RED,
                ConsoleMessageStatus::Notify => COLOR::back::GREEN,
            };
            
            queue!(
                stdout(), 
                MoveTo(0, (self.terminal_height - num_line - 1) as u16), 
                Clear(ClearType::FromCursorDown), 
                MoveDown(1), 
                Print(format!("{}{}{}{}", color, message, blank, COLOR::RESET))
            )?;
        }
        stdout().flush()?;
        Ok(())
    }
}
