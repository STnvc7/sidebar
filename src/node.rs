use std::path::PathBuf;
use std::io::stdout;

use crossterm::{cursor, execute, queue, style};
use crossterm::style::{Print, Color, SetForegroundColor};

#[derive(Debug, PartialEq)]
enum NodeType{
    Folder,
    File,
}

//ファイルの木構造を構成するノード
#[derive(Debug)]
pub struct Node{
    name    : String,
    path    : PathBuf,
    childs  : Vec<Box<Node>>,
    node_type    : NodeType,

    num_childs : usize,
    selected_child : usize,

    opened  : bool,
    selected: bool,
    ignore  : bool,
}
//----------------------------------

//最初のノードを作る関数
pub fn new_node(target: PathBuf) -> Result<Box<Node>, String>{

    if !target.is_dir(){
        return Err(String::from(format!("Error : {} is not a folder", target.into_os_string().into_string().unwrap())))
    }

    let new_node = Box::new(Node{
                    name: target.file_name().unwrap().to_string_lossy().into_owned(),
                    path: target,
                    node_type: NodeType::Folder,
                    childs: Vec::new(),
                    num_childs: 0,
                    selected_child: 0,
                    opened: false,
                    selected: true,
                    ignore: false,
                    });
    return Ok(new_node)
}

pub fn build_tree(target: &PathBuf) -> Result<Box<Node>, String>{

    //ノードの配下にあるディレクトリまたはファイルを取得
    let files = match target.read_dir(){
        Ok(v) => {v}
        Err(E) => {return Err(E.to_string())}
    };

    let num_childs = target.read_dir().unwrap().count();
    let mut _childs = Vec::new();

    //ノード配下のディレクトリまたはファイルを一つずつ見ていく
    for dir_entry in files{

        let _path = dir_entry.unwrap().path();      //パスを取得  
        let _filename = _path.file_name().unwrap().to_string_lossy().into_owned();  //取得したパスからファイル名を抜き出してString型へ

        let _child = if _path.is_dir(){ build_tree(&_path).unwrap() }
                     else{
                        Box::new(Node{
                            name: _filename,
                            path: _path,
                            node_type: NodeType::File,
                            childs: Vec::new(),

                            num_childs: 0,
                            selected_child: 0,

                            opened: false,
                            selected: false,
                            ignore: false,
                     })};
        _childs.push(Box::new(_child));
    }

    let new_node = Box::new(Node{
                    name: target.file_name().unwrap().to_string_lossy().into_owned(),
                    path: target.to_path_buf(),
                    node_type: NodeType::Folder,
                    childs: Vec::new(),

                    num_childs: num_childs,
                    selected_child: 0,

                    opened: false,
                    selected: false,
                    ignore: false,
    });
    return Ok(new_node)
}

//ノードのメソッド
impl Node{

    pub fn is_opened(&self) -> bool{
        return self.opened
    }

    pub fn select_down(&mut self){

        if self.num_childs == 0{
            return
        }

        //そのノードそのものが選択されている時 -> 子ノードの一番最初のノードを選択    
        if self.selected{
            if self.childs.len() == 0{
                return
            }
            self.childs[0].selected = true;
            self.selected = false;
        }

        //そのノードが選択されていない時 ->  子ノードの中から現在選択されているノードを取得し，
        else{
            if self.childs[self.selected_child].opened{
                if self.childs[self.selected_child].selected_child == self.childs[self.selected_child].num_childs - 1{
                    self.childs[self.selected_child].selected = false;
                    self.selected_child += 1;
                    self.childs[self.selected_child].selected = true;
                }
                self.childs[self.selected_child].select_down();
            }
            else{
                if self.selected_child != self.num_childs - 1{
                    self.childs[self.selected_child].selected = false;
                    self.selected_child += 1;
                    self.childs[self.selected_child].selected = true;
                }
            }

        }
    }

    pub fn open_node(&mut self){
        self.opened = true;
        return
    }

    // pub fn open_node(&mut self){
    //     // execute!(stdout(), cursor::SavePosition, cursor::MoveTo(0, 20), Print(format!("node open {}", self.name)), cursor::RestorePosition);
    //     if !self.selected{
    //         for child in self.childs.iter_mut(){
    //             if child.selected{
    //                 child.open_node();
    //             }
    //         }
    //         return
    //     }

    //     if self.opened{
    //         self.close_node();
    //         return
    //     }

    //     //ノードの配下にあるディレクトリまたはファイルを取得
    //     let files = match self.path.read_dir(){
    //         Ok(v) => {v}
    //         Err(E) => {return}
    //     };

    //     self.opened = true;    //Node構造体のopen属性をtrueに
    //     self.num_childs = self.path.read_dir().unwrap().count();

    //     //ノード配下のディレクトリまたはファイルを一つずつ見ていく
    //     for dir_entry in files{

    //         let _path = dir_entry.unwrap().path();      //パスを取得  
    //         let _filename = _path.file_name().unwrap().to_string_lossy().into_owned();  //取得したパスからファイル名を抜き出してString型へ

    //         let node = if _path.is_dir(){               //ディレクトリの場合
    //             Node{
    //                 name: _filename,
    //                 path: _path,
    //                 node_type: NodeType::Folder,
    //                 childs: Vec::new(),

    //                 num_childs: 0,
    //                 selected_child: 0,

    //                 opened: false,
    //                 selected: false,
    //                 ignore: false,
    //                 }
    //         }
    //         else{
    //             Node{                                   //ファイルの場合
    //                 name: _filename,
    //                 path: _path,
    //                 node_type: NodeType::File,
    //                 childs: Vec::new(),

    //                 num_childs: 0,
    //                 selected_child: 0,

    //                 opened: false,
    //                 selected: false,
    //                 ignore: false,
    //                 }
    //         };
    //         self.childs.push(Box::new(node));
    //     }

    //     return
    // }

    // pub fn close_node(&mut self){
    //     self.childs = Vec::new();
    //     self.opened = false;
    //     self.num_childs = 0;
    // }

    pub fn print_tree(&self, rank : usize){

        //ルートの時はカーソルを一番上に移動
        if rank == 0{
            execute!(stdout(), cursor::MoveTo(0,0));
        }

        //このノードの情報を出力
        let name = &self.name;               //ファイル/フォルダ名
        let indent = String::from("   ");   //成形用のインデントの元　改装に応じてrepeatさせる

        //選択されているノードは緑で表示
        let output_color = if self.selected {"\x1b[32m"} else {"\x1b[37m"};

        //ノードの種類ごとで出力形式を分ける
        let output = match &self.node_type{
            NodeType::Folder => format!("{}> {}",indent.repeat(rank), name),    //フォルダの時は >マークを先頭につける
            NodeType::File => format!("{}{}",indent.repeat(rank), name),
            };

        //出力
        print!("{}{}", output_color, output);
        execute!(stdout(), cursor::MoveToNextLine(1));

        //ノードが開かれているときは子ノードを展開して再起的に出力
        //tree.childがNoneの時はreturnして再帰を終了
        if self.opened{
            for child in self.childs.iter(){
                child.print_tree(rank+1);
            }
        }
    }
}

