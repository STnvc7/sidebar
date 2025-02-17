use crate::color as COLOR;
use crate::icon;
use crate::node::NodeType;
use crate::node_map::NodeMap;
use crate::config::Config;

use anyhow::Result;
use std::cell::RefCell;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::rc::Rc;
use uuid::Uuid;

use crossterm::cursor::{MoveTo, MoveToNextLine};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{queue, terminal};

#[allow(dead_code)]
pub struct Viewer {
    node_map: Rc<RefCell<NodeMap>>,
    id_list: Vec<Uuid>,
    console_message: Option<String>,
    display_start_idx: usize,
    cursor_idx: usize,
    secondoy_cursor_idx: Option<usize>,
    terminal_width: usize,
    terminal_height: usize,
    config: Rc<Config>,
}

#[allow(dead_code)]
impl Viewer {
    // ----------------------------------------------------------------
    // コンストラクタ
    // ----------------------------------------------------------------
    pub fn new(node_map: Rc<RefCell<NodeMap>>, config: Rc<Config>) -> Viewer {
        let (width, height) = terminal::size().unwrap();
        let id_list: Vec<Uuid> = Vec::new();

        Viewer {
            node_map: node_map,
            id_list: id_list,
            console_message: None,
            display_start_idx: 0,
            cursor_idx: 0,
            secondoy_cursor_idx: None,
            terminal_width: width as usize,
            terminal_height: height as usize,
            config: config,
        }
    }

    // カーソルが選択しているノードのIDを取得
    pub fn get_cursor_id(&self) -> Uuid {
        let current_idx = match self.secondoy_cursor_idx {
            Some(i) => i,
            None => self.cursor_idx,
        };
        self.id_list[current_idx].clone()
    }

    // コンソールメッセージを保存
    pub fn set_console_message(&mut self, message: String) {
        self.console_message = Some(message);
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

    // カーソルを上に -----------------------
    pub fn cursor_up(&mut self) {
        // セカンダリーカーソルがあるならそっち優先
        let mut current_idx = match self.secondoy_cursor_idx {
            Some(i) => i,
            None => self.cursor_idx,
        };

        // 更新------------------
        if current_idx == 0 {
            return;
        }
        current_idx -= 1;

        // 格納 -----------------
        match self.secondoy_cursor_idx {
            Some(_) => {self.secondoy_cursor_idx = Some(current_idx);}
            None => {self.cursor_idx = current_idx}
        }

        return;
    }

    // カーソルを下に -----------------------
    pub fn cursor_down(&mut self) {
        // セカンダリーカーソルがあるならそっち優先
        let mut current_idx = match self.secondoy_cursor_idx {
            Some(i) => i,
            None => self.cursor_idx,
        };
        let length = self.id_list.len();

        // 更新------------------
        if current_idx >= length-1 {
            return;
        }
        current_idx += 1;

        // 格納 -----------------
        match self.secondoy_cursor_idx {
            Some(_) => {self.secondoy_cursor_idx = Some(current_idx);}
            None => {self.cursor_idx = current_idx}
        }
        return;
    }

    // ランクの異なるノードまでカーソルを上向きにジャンプ ---------------------
    pub fn cursor_jump_up(&mut self) -> Result<()> {
        let node_map = self.node_map.borrow();
        let mut current_idx = match self.secondoy_cursor_idx {
            Some(i) => i,
            None => self.cursor_idx,
        };
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
                match self.secondoy_cursor_idx {
                    Some(_) => {self.secondoy_cursor_idx = Some(current_idx);}
                    None => {self.cursor_idx = current_idx}
                }
                break;
            }
        }
        return Ok(());
    }

    // ランクの異なるノードまでカーソルを下向きにジャンプ ---------------------
    pub fn cursor_jump_down(&mut self) -> Result<()> {
        let node_map = self.node_map.borrow();

        let mut current_idx = match self.secondoy_cursor_idx {
            Some(i) => i,
            None => self.cursor_idx,
        };
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
                match self.secondoy_cursor_idx {
                    Some(_) => {self.secondoy_cursor_idx = Some(current_idx);}
                    None => {self.cursor_idx = current_idx}
                }
                break;
            }
        }
        return Ok(());
    }

    pub fn activate_secondly_cursor_idx(&mut self) {
        self.secondoy_cursor_idx = Some(self.cursor_idx)
    }
    pub fn deactivate_secondly_cursor_idx(&mut self) {
        self.secondoy_cursor_idx = None
    }

    // node_mapと同期 -------------------------
    pub fn sync(&mut self) -> Result<()> {
        let node_map = self.node_map.borrow();

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
    // セカンダリーカーソルのノード => 緑
    fn get_line_color(&self, i: usize) -> &str {
        if i == self.cursor_idx {return COLOR::BLUE}
        else {
            match self.secondoy_cursor_idx {
                Some(c) => {
                    if c == i {return COLOR::GREEN}
                    else {return COLOR::RESET}
                }
                None => {return COLOR::RESET}
            }
        }
    }

    // 表示開始位置の更新
    fn update_display_start(&mut self) {
        let cursor_idx = match self.secondoy_cursor_idx {
            Some(i) => i,
            None => self.cursor_idx,
        };
        
        if cursor_idx >= self.display_start_idx + self.terminal_height - 1 {
            // 先に1を足しておかないとusizeが一瞬負の値になってパニックする
            self.display_start_idx = (cursor_idx + 1) - self.terminal_height;
        }
        else if cursor_idx < self.display_start_idx{
            self.display_start_idx = cursor_idx;
        }
    }

    // 表示をおこなうメソッド -----------------------------------------
    pub fn display(&mut self) -> Result<()> {
        self.update_display_start();
        
        let start = self.display_start_idx;
        let end = start + self.terminal_height;

        let node_map = self.node_map.borrow();
        queue!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;

        for i in start..end {
            if i >= self.id_list.len() {
                break;
            }
            let color = self.get_line_color(i);
            let id = self.id_list[i];
            let name = node_map.get_name(&id)?;
            let rank = node_map.get_rank(&id)?;
            let node_type = node_map.get_node_type(&id)?;
            let icon = match node_type {
                NodeType::Folder => {
                    let is_open = node_map.has_children(&id)?;
                    icon::get_folder_icon(is_open, self.config.nerd_font)
                }
                NodeType::File => {
                    icon::get_file_icon(&name, self.config.nerd_font)
                }
            };
            let line = self.format(name, icon, rank, color);
            let out = format!("{}{}", COLOR::RESET, line);

            queue!(stdout(), Print(out), MoveToNextLine(1))?;
        }

        if let Some(ref m) = self.console_message {
            let num_line = m.len().div_ceil(self.terminal_width);
            queue!(stdout(), MoveTo(0, (self.terminal_height-num_line) as u16), Print(format!("{}{}{}", COLOR::RED, m, COLOR::RESET)))?;
        }
        stdout().flush()?;
        Ok(())
    }
}
