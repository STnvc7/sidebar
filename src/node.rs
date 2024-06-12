use std::path::PathBuf;
use std::collections::VecDeque;
use std::cmp::Ordering;

use crate::viewer::{TextElement, new_element, get_num_lines, RIGHT_MARGIN};

pub enum NodeType{
    Folder,
    File,
}

//ファイルの木構造を構成するノード
#[allow(dead_code)]
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
pub fn new(target: &PathBuf) -> Box<Node>{

    //ノードの配下にあるディレクトリまたはファイルを取得
    let _childs = get_childs(target);
    let num_childs = target.read_dir().unwrap().count();

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

fn get_childs(target : &PathBuf) -> Vec<Box<Node>>{
    let files = target.read_dir().unwrap();
    let mut childs = Vec::new();

    for dir_entry in files{

        let _path = dir_entry.unwrap().path();      //パスを取得  
        let _filename = _path.file_name().unwrap().to_string_lossy().into_owned();  //取得したパスからファイル名を抜き出してString型へ
        let _node_type = if _path.is_dir() {NodeType::Folder} else{NodeType::File};
        let _child = Box::new(Node{
                                name: _filename,
                                path: _path,
                                node_type: _node_type,
                                childs: Vec::new(),
                                num_childs: 0,
                                opened: false,
                                ignore: false,});
        childs.push(_child);
    }

    //ディレクトリ->ファイルの順にソート その後名前順でソート
    childs = sort_node(childs);

    return childs
}

fn sort_node(mut nodes : Vec<Box<Node>>) -> Vec<Box<Node>>{

    nodes.sort_by(|a, b|{
        match a.node_type{
            NodeType::Folder => {
                match b.node_type{
                    NodeType::Folder => Ordering::Equal,
                    NodeType::File => Ordering::Less}}
            NodeType::File => {
                match b.node_type{
                    NodeType::Folder => Ordering::Greater,
                    NodeType::File => Ordering::Equal}}
        }.then(a.name.cmp(&b.name))
        });

    return nodes
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

    fn set_childs(&mut self, childs: Vec<Box<Node>>){
        self.num_childs = childs.len();
        self.childs = childs;
    }


    //Viewerで選択されているノードのルートを受け取り，パスを表示　　ルート：ノードの木の根っこから選択されているノードへのパス
    pub fn get_path(&self, mut route: VecDeque<usize>) -> PathBuf{

        let _path = if route.len()!=0 {
            let poped_node_idx = route.pop_front().unwrap();
            self.childs[poped_node_idx].get_path(route)
        }
        else{
            self.path.clone()
        };

        return _path
    }

    // fn get_open_status(&self, route : VecDeque<usize>) -> Vec<VecDeque<usize>>{

    //     routes = Vec::new();

    //     for (i, child) in self.childs.iter().enumerate(){
    //         if child.opened{
    //             let mut new_route = route.clone();
    //             new_route.push_back(i as usize);
    //             let mut _route = child.get_open_status(new_route);
    //             routes.append(_route);
    //         }
    //     }

    //     return routes

    // }

    pub fn update_node(&mut self, route : VecDeque<usize>){

        let open_only = false;
        self.open_node(route.clone(), open_only);
        self.open_node(route.clone(), open_only);
    }

    //-----------------------------------------------------------------------------------------
    pub fn open_node(&mut self, mut route: VecDeque<usize>, open_only : bool){
        //route : 現在選択されているノードの経路

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
                match self.childs[poped_node_idx].node_type{
                    NodeType::Folder => {
                        let _childs = get_childs(&self.childs[poped_node_idx].path);
                        self.childs[poped_node_idx].set_childs(_childs);
                        self.childs[poped_node_idx].set_opened(true);
                    }
                    NodeType::File =>{}
                }
            }
            else {
                if open_only{
                    return
                }
                self.childs[poped_node_idx].set_opened_all(false);
            }
            return
        }

        self.childs[poped_node_idx].open_node(route.clone(), open_only);
        return
    }

    //-----------------------------------------------------------------------------------------
    //木の出力用のString型のVecDequeを返す
    pub fn format(&self) -> VecDeque<TextElement> {
        let texts = self.format_for_viewer(0, VecDeque::new());
        return texts
    }

    fn format_for_viewer(&self, rank : usize, route: VecDeque<usize>) -> VecDeque<TextElement>{

        let mut _output = VecDeque::new();

        if self.opened{
            for (i, child) in self.childs.iter().enumerate(){

                let mut new_route = route.clone();
                new_route.push_back(i as usize);
                let mut _child_elem = child.format_for_viewer(rank+1, new_route);

                _output.append(&mut _child_elem);
            }
        }

        //このノードの情報
        let _name        = self.name.to_string();
        let _path        = self.path.clone();
        let _is_opened   = self.opened;

        let output_elem : TextElement;
        match self.node_type{
            NodeType::Folder => {
                let _num_lines = get_num_lines(&_name, &(&rank*2), &RIGHT_MARGIN);
                output_elem = new_element(_name, _num_lines, NodeType::Folder, _is_opened, rank, route);},
            NodeType::File   => {
                let _num_lines = get_num_lines(&_name, &(&rank*2), &RIGHT_MARGIN);
                output_elem = new_element(_name, _num_lines, NodeType::File, false, rank, route)}
        };
        _output.push_front(output_elem);

        return _output
    }
}

