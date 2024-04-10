use std::path::PathBuf;
use std::io::stdout;
use std::collections::VecDeque;

use crate::color;

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

    opened  : bool,
    selected: bool,
    ignore  : bool,
}
//----------------------------------

//最初の木を構築する関数
pub fn build_tree(target: &PathBuf) -> Box<Node>{

    //ノードの配下にあるディレクトリまたはファイルを取得
    let files = target.read_dir().unwrap();

    let num_childs = target.read_dir().unwrap().count();
    let mut _childs = Vec::new();

    //ノード配下のディレクトリまたはファイルを一つずつ見ていく
    for dir_entry in files{

        let _path = dir_entry.unwrap().path();      //パスを取得  
        let _filename = _path.file_name().unwrap().to_string_lossy().into_owned();  //取得したパスからファイル名を抜き出してString型へ
        let _child = if _path.is_dir(){ build_tree(&_path) }
                     else{
                            Box::new(Node{
                            name: _filename,
                            path: _path,
                            node_type: NodeType::File,
                            childs: Vec::new(),

                            num_childs: 0,

                            opened: false,
                            selected: false,
                            ignore: false,})};
        _childs.push(_child);
    }

    let new_node = Box::new(Node{
                    name: target.file_name().unwrap().to_string_lossy().into_owned(),
                    path: target.to_path_buf(),
                    node_type: NodeType::Folder,
                    childs: _childs,

                    num_childs: num_childs,

                    opened: false,
                    selected: false,
                    ignore: false,
    });
    return new_node
}

//ノードのメソッド
impl Node{

    //セッター
    pub fn set_opened(&mut self, value : bool){
        self.opened = value;
    }

    pub fn set_selected(&mut self, value: bool){
        self.selected = value;
    }

    pub fn set_selected_all(&mut self, value: bool){
        self.selected = value;

        if self.num_childs == 0{
            return
        }

        for child in self.childs.iter_mut(){
            child.set_selected_all(value);
        }
    }

    pub fn set_opened_all(&mut self, value: bool){
        self.opened = value;

        if self.num_childs == 0{
            return
        }

        for child in self.childs.iter_mut(){
            child.set_opened_all(value);
        }
    }

    pub fn open_node(&mut self, route: VecDeque<usize>){

        let result = route.pop_front();
        let poped_node_idx : usize;
        match result{
            Some(v) => { poped_node_idx = v;}
            None    => { return }
        }

        if route.len() == 0{
            if self.childs[poped_node_idx].opened == false{
                self.childs[poped_node_idx].opened = true;
            }
            else {
                self.childs[poped_node_idx].opened = false;
            }
            return
        }

        self.childs[poped_node_idx].open_node(route.clone());
        return
    }

    //木の出力用のString型のVecDequeを返す
    pub fn convert_to_string_vec(&self, rank : usize) -> VecDeque<String>{

        //このノードの情報を出力
        let name = &self.name;               //ファイル/フォルダ名
        let indent = String::from("   ");   //成形用のインデントの元　改装に応じてrepeatさせる

        //ノードの種類ごとで出力形式を分ける
        let output_line = match &self.node_type{
            NodeType::Folder => format!("{}> {}",indent.repeat(rank), name),    //フォルダの時は >マークを先頭につける
            NodeType::File => format!("{}{}",indent.repeat(rank), name),
            };

        let mut _output = VecDeque::new();
        if self.opened{
            for child in self.childs.iter(){
                let mut _line = child.convert_to_string_vec(rank+1);
                _output.append(&mut _line);
            }
            _output.push_front(output_line);
        }
        else{
            _output.push_back(output_line);
        }

        return _output
    }
}

