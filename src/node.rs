use std::path::PathBuf;
use std::io::stdout;
use std::collections::VecDeque;


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

    //ゲッター
    pub fn get_num_childs(&self) -> usize{
        return self.num_childs
    }

    pub fn is_opened(&self) -> bool{
        return self.opened
    }

    pub fn route_node(&mut self, mut route: VecDeque<usize>){

        let result = route.pop_front();
        let idx = match result{
            Some(v) => {v}
            None => {return}
        };
        
        self.childs[idx].opened = true;  

        self.childs[idx].route_node(route)
    }

    // pub fn select_down(&mut self){

    //     if self.num_childs == 0{
    //         return
    //     }

    //     //そのノードそのものが選択されている時 -> 子ノードの一番最初のノードを選択    
    //     if self.selected{
    //         if self.childs.len() == 0{
    //             return
    //         }
    //         self.childs[0].selected = true;
    //         self.selected = false;
    //     }

    //     //そのノードが選択されていない時 ->  子ノードの中から現在選択されているノードを取得し，
    //     else{
    //         if self.childs[self.selected_child].opened{
    //             if self.childs[self.selected_child].selected_child == self.childs[self.selected_child].num_childs - 1{
    //                 self.childs[self.selected_child].selected = false;
    //                 self.selected_child += 1;
    //                 self.childs[self.selected_child].selected = true;
    //             }
    //             self.childs[self.selected_child].select_down();
    //         }
    //         else{
    //             if self.selected_child != self.num_childs - 1{
    //                 self.childs[self.selected_child].selected = false;
    //                 self.selected_child += 1;
    //                 self.childs[self.selected_child].selected = true;
    //             }
    //         }

    //     }
    // }

    pub fn close_node(&mut self){
        self.opened = false;
        return
    }

    pub fn open_node(&mut self){
    
        if !self.selected{
            for child in self.childs.iter_mut(){
                if child.selected{
                    child.open_node();
                }
            }
            return
        }
        else{
            if self.opened{
                self.close_node();
                return
            }
            else{
                self.opened = true;
            }
        }
    }

    //木の出力用のString型のVecDequeを返す
    pub fn convert_to_string_vec(&self, rank : usize) -> VecDeque<String>{

        //このノードの情報を出力
        let name = &self.name;               //ファイル/フォルダ名
        let indent = String::from("   ");   //成形用のインデントの元　改装に応じてrepeatさせる

        //選択されているノードは緑で表示
        let output_color = if self.selected {"\x1b[32m"} else {"\x1b[37m"};

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

