use std::path::PathBuf;

use std::io::stdout;
use crossterm::{
    cursor, execute, ExecutableCommand, terminal,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    event::{read, Event, KeyCode},
};

use crate::cursor_control;

#[derive(Debug, PartialEq)]
enum NodeType{
    Folder,
    File,
}

#[derive(Debug)]
pub struct Node{
    name    : String,
    path    : PathBuf,
    childs  : Option<Vec<Box<Node>>>,
    node_type    : NodeType,
    opened  : bool,
    selected: bool,
    ignore  : bool,
}

impl Node{

    pub fn open_node(&mut self){
        
        self.opened = true;                             //Node構造体のopen属性をtrueに

        let files = self.path.read_dir().unwrap();      //ノードの配下にあるディレクトリまたはファイルを取得
        let mut childs : Vec<Box<Node>> = Vec::new();   //ノードの配下のノードを保管するためのベクタを初期化

        //ノード配下のディレクトリまたはファイルを一つずつ見ていく
        for dir_entry in files{

            let _path = dir_entry.unwrap().path();      //パスを取得  
            let _filename = _path.file_name().unwrap().to_string_lossy().into_owned();  //取得したパスからファイル名を抜き出してString型へ

            let node = if _path.is_dir(){               //ディレクトリの場合
                Node{
                    name: _filename,
                    path: _path,
                    node_type: NodeType::Folder,
                    childs: None,
                    opened: false,
                    selected: false,
                    ignore: false,
                    }
            }
            else{
                Node{                                   //ファイルの場合
                    name: _filename,
                    path: _path,
                    node_type: NodeType::File,
                    childs: None,
                    opened: false,
                    selected: false,
                    ignore: false,
                    }
            };
            childs.push(Box::new(node));
        }

        self.childs = Some(childs);

        return
    }

    pub fn print_tree(&self, rank : usize){

        //ルートの時はカーソルを一番上に移動
        if rank == 0{
            execute!(stdout(), cursor::MoveTo(0,0));
        }

        cursor_control::to_edge_cursor();

        //このノードの情報を出力
        let name = &self.name;               //ファイル/フォルダ名
        let indent = String::from("   ");   //成形用のインデントの元　改装に応じてrepeatさせる

        let output_color = if self.opened{SetForegroundColor(Color::Green)} else {SetForegroundColor(Color::Black)};

        //ノードの種類ごとで出力形式を分ける
        let output = match &self.node_type{
            NodeType::Folder => format!("{}> {}\n",indent.repeat(rank), name),    //フォルダの時は >マークを先頭につける
            NodeType::File => format!("{}{}\n",indent.repeat(rank), name),
            };
        
        execute!(stdout(), output_color, Print(output));
        
        //ノードが開かれているときは子ノードを展開して再起的に出力
        //tree.childがNoneの時はreturnして再帰を終了
        if self.opened{
            let childs: &Vec<Box<Node>>;
            match &self.childs{
                Some(_v) =>{childs = &_v}
                None => return
            }
            for child in childs.iter(){
                child.print_tree(rank+1);
            }
        }
    }
}

pub fn new_node(target: PathBuf) -> Result<Box<Node>, String>{

    if !target.is_dir(){
        return Err(String::from(format!("Error : {} is not a folder", target.into_os_string().into_string().unwrap())))
    }

    let new_node = Box::new(Node{
                    name: target.file_name().unwrap().to_string_lossy().into_owned(),
                    path: target,
                    node_type: NodeType::Folder,
                    childs: None,
                    opened: false,
                    selected: true,
                    ignore: false,
                    });

    return Ok(new_node)
}
