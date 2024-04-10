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

    //上下ボタンが押されたときの遷移先を示す配列を返す関数 経路が変わる時はSome(), 変わらない時はNoneを返す
    pub fn get_route(&self, mut route: VecDeque<usize>, direction: &str) -> Option<VecDeque<usize>>{

        //返り値用の新しい経路
        let mut new_route = VecDeque::new();

        //ノードが開かれていない場合は何もしない
        if !self.opened {
            return None
        }
        
        match direction{

            //下キーが押された時
            "down" => {
                // 元の経路がない時 -> 一番最初の子ノードを選択して返す． 子ノードがない場合は何も返さない
                if route.len() == 0{
                    if self.num_childs != 0{
                        new_route.push_back(0);
                        return Some(new_route)
                    }
                    return None
                }

                //  元の遷移経路の先頭から値をポップ
                let mut poped_node_idx : usize = route.pop_front().unwrap();

                // ポップしたインデックスの子ノードが開かれている時 -> 再起的に遷移先を取得
                if self.childs[poped_node_idx].opened {
                    let result = self.childs[poped_node_idx].get_route(route.clone(), direction);

                    match result{
                        //子ノードから遷移先を取得できた時 ->  新しい経路とくっ付ける
                        Some(mut v) => {new_route.append(&mut v);}

                        //子ノードから遷移先を取得できない(子ノードの最後まで達した時) -> 次の子ノードに移動
                        None    => {
                            if poped_node_idx == (self.num_childs - 1){ return None }
                            else{ poped_node_idx += 1;}}
                    }
                    new_route.push_front(poped_node_idx);
                    return Some(new_route)
                }

                //ポップしたインデックスの小ノードが開かれていない時 -> ポップしたノードの一つ下のノードに遷移
                else{
                    if poped_node_idx == (self.num_childs - 1){
                        return None
                    }
                    else {
                        new_route.push_back(poped_node_idx + 1);
                        return Some(new_route);
                    }
                }
            }

            "up" => {
                if route.len() == 0{
                    return None
                }

                let mut poped_node_idx = route.pop_back().unwrap();

                if poped_node_idx == 0 {
                    if route.len() == 0 { return None }
                    return Some(route)
                }   
                else {
                    route.push_back(poped_node_idx - 1);
                    
                    return Some(route)
                }
            }

            _ => {return None}
        }
    }

    pub fn set_route(&mut self, mut route: VecDeque<usize>){

        let result = route.pop_front();

        match result {
            Some(v) => {
                if route.len() != 0 {
                    self.childs[v].set_route(route.clone());
                }
                else{
                    self.childs[v].selected = true;
                }
            }
            None => {}
        }
    }

    pub fn close_node(&mut self){
        self.opened = false;
        return
    }

    pub fn open_node(&mut self){

        if self.selected {
            if self.opened { self.close_node(); } else { self.opened = true; }
        }
        else{
            for child in self.childs.iter_mut(){
                child.open_node();
            }
        }
    }

    //木の出力用のString型のVecDequeを返す
    pub fn convert_to_string_vec(&self, rank : usize) -> VecDeque<String>{

        //このノードの情報を出力
        let name = &self.name;               //ファイル/フォルダ名
        let indent = String::from("   ");   //成形用のインデントの元　改装に応じてrepeatさせる

        //選択されているノードは緑で表示
        let output_color = if self.selected {color::GREEN} else {color::WHITE};

        //ノードの種類ごとで出力形式を分ける
        let output_line = match &self.node_type{
            NodeType::Folder => format!("{}{}> {}",output_color, indent.repeat(rank), name),    //フォルダの時は >マークを先頭につける
            NodeType::File => format!("{}{}{}",output_color, indent.repeat(rank), name),
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

