use std::path::PathBuf;
use std::collections::VecDeque;

use crate::text_line::TextElement;

#[derive(Debug, PartialEq)]
pub enum NodeType{
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
    ignore  : bool,
}
//-----------------------------------------------------------------------------------------

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
                    ignore: false,
    });
    return new_node
}
//-----------------------------------------------------------------------------------------

//ノードのメソッド
impl Node{

    //セッター
    fn set_opened(&mut self, value : bool){
        self.opened = value;
    }

    fn set_opened_all(&mut self, value: bool){
        self.set_opened(value);

        if self.num_childs == 0{
            return
        }

        for child in self.childs.iter_mut(){
            child.set_opened_all(value);
        }
    }
    //-----------------------------------------------------------------------------------------
    pub fn open_node(&mut self, mut route: VecDeque<usize>){

        let result = route.pop_front();
        let poped_node_idx : usize;

        match result{
            Some(v) => { poped_node_idx = v;}

            //ルートのノードの場合
            None    => {if self.opened == false {
                            self.set_opened(true);
                        }
                        else {
                            self.set_opened_all(false);
                        }
                        return }
        }

        if route.len() == 0{
            if self.childs[poped_node_idx].opened == false{
                self.childs[poped_node_idx].set_opened(true);
            }
            else {
                self.childs[poped_node_idx].set_opened_all(false);
            }
            return
        }

        self.childs[poped_node_idx].open_node(route.clone());
        return
    }

    //-----------------------------------------------------------------------------------------
    //木の出力用のString型のVecDequeを返す
    pub fn format(&self) -> VecDeque<TextElement> {
        let texts = self.format_for_textline(0, VecDeque::new());
        return texts
    }
    fn format_for_textline(&self, rank : usize, route: VecDeque<usize>) -> VecDeque<TextElement>{

        let mut _output = VecDeque::new();

        if self.opened{
            for (i, child) in self.childs.iter().enumerate(){

                let mut new_route = route.clone();
                new_route.push_back(i as usize);
                let mut _child_elem = child.format_for_textline(rank+1, new_route);

                _output.append(&mut _child_elem);
            }
        }

        //このノードの情報
        let _name        = self.name.to_string();
        let _path        = self.path.clone();
        let output_elem =  match self.node_type{
            NodeType::Folder => {TextElement{ text : _name, node_type : NodeType::Folder, path : _path,
                                              rank : rank, route : route}}
            NodeType::File   => {TextElement{ text : _name, node_type : NodeType::File, path : _path,
                                              rank : rank, route : route}}
        };
        _output.push_front(output_elem);

        return _output
    }
}

