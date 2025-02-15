use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub enum NodeType {
    Folder,
    File,
}

#[derive(Debug)]
pub struct Node {
    name: String,
    path: PathBuf,
    node_type: NodeType,
    rank: usize,
    children: Option<Vec<Uuid>>,
}

impl Node {
    pub fn new(path: PathBuf, node_type: NodeType, rank: usize) -> Node {
        Node {
            name: path.file_name().unwrap().to_string_lossy().into_owned(),
            path: path,
            node_type: node_type,
            rank: rank,
            children: None,
        }
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
    pub fn get_children_ids(&self) -> Option<Vec<Uuid>> {
        self.children.clone()
    }
    pub fn set_children_ids(&mut self, children: Option<Vec<Uuid>>) {
        self.children = children;
    }
}
