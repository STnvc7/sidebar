use std::path::PathBuf;
use std::cmp::Ordering;
use uuid::Uuid;

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub enum NodeType {
    Folder,
    File,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Node {
    id: Uuid,
    name: String,
    path: PathBuf,
    node_type: NodeType,
    rank: usize,
    children: Option<Vec<Uuid>>,
    is_open: bool,
}

impl Node {
    pub fn new(id: Uuid, path: PathBuf, node_type: NodeType, rank: usize) -> Node {
        Node {
            id: id,
            name: path.file_name().unwrap().to_string_lossy().into_owned(),
            path: path,
            node_type: node_type,
            rank: rank,
            children: None,
            is_open: false,
        }
    }

    pub fn sort_fn(a: &Node, b: &Node) -> Ordering {
        let path_a = a.get_path();
        let file_type_a = a.get_node_type();
        let path_b = b.get_path();
        let file_type_b = b.get_node_type();

        match file_type_a.cmp(&file_type_b) {
            Ordering::Equal => {
                // 拡張子で比較
                let ext_a = path_a.extension();
                let ext_b = path_b.extension();

                if (ext_a == None) || (ext_b == None) {
                    return path_a.file_name().cmp(&path_b.file_name());
                }

                match ext_a.unwrap().cmp(ext_b.unwrap()) {
                    Ordering::Equal => return path_a.file_name().cmp(&path_b.file_name()),
                    other => return other,
                }
            }
            other => return other,
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id.clone()
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
    pub fn get_rank(&self) -> usize {
        self.rank
    }
    pub fn get_node_type(&self) -> NodeType {
        self.node_type.clone()
    }
    pub fn get_is_open(&self) -> bool {
        self.is_open
    }
    pub fn get_children_ids(&self) -> Option<Vec<Uuid>> {
        self.children.clone()
    }
    pub fn set_children_ids(&mut self, children: Option<Vec<Uuid>>) {
        self.children = children;
    }
    pub fn set_is_open(&mut self, is_open: bool) {
        self.is_open = is_open;
    }
}