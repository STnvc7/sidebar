#![allow(unused_imports)]

use anyhow::{anyhow, Result};
use log;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_dir;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;
use globset::{Glob, GlobSet, GlobSetBuilder};

use crate::node::{Node, NodeType};
use crate::utils::path::get_file_type;
use crate::config::Config;

#[derive(Debug)]
pub struct NodeMap {
    node_map: HashMap<Uuid, Node>,
    root_id: Uuid,
    config: Arc<Config>,
}

#[allow(dead_code)]
impl NodeMap {
    // ----------------------------------------------------------------
    // コンストラクタ
    // ----------------------------------------------------------------
    pub fn new(root: PathBuf, config: Arc<Config>) -> Result<NodeMap> {
        let root_id = Uuid::new_v4();
        let root_node = Node::new(root_id.clone(), root, NodeType::Folder, 0)?;

        let mut node_map: HashMap<Uuid, Node> = HashMap::new();
        node_map.insert(root_id.clone(), root_node);

        return Ok(NodeMap {
            node_map: node_map,
            root_id: root_id,
            config: config,
        })
    }

    // ----------------------------------------------------------------
    // 閉じてたら子ノードを追加，開いてたら子ノードを閉じる
    // ----------------------------------------------------------------
    pub fn open_and_close_node(&mut self, id: &Uuid) -> Result<()> {

        // 開かれているとき
        if self.get_is_open(id)? {
            // 省メモリモードの時は子ノードを削除
            if self.config.saving_memory {
                if let Some(children_ids) = self.get_children_ids(id)?{
                    for children_id in children_ids.iter() {
                        self.delete_node(&children_id)?;
                    }
                    self.set_children_ids(id, None)?;
                }
            }
            self.set_is_open(id, false)?;
        } 
        // 閉じているとき
        else {
            // 子ノードが無い時は子ノードを追加
            if let None = self.get_children_ids(id)?{
                self.add_children(id)?;
            }
            self.set_is_open(id, true)?;
        }

        return Ok(());
    }

    // ----------------------------------------------------------------
    // ノードを削除
    // ----------------------------------------------------------------
    fn delete_node(&mut self, id: &Uuid) -> Result<()> {
        // 子ノードがある場合は再帰的に削除
        let children_ids = self.get_children_ids(id)?;
        if let Some(c_ids) = children_ids {
            for c_id in c_ids.iter() {
                self.delete_node(&c_id)?;
            }
        }

        self.node_map.remove(id);
        return Ok(());
    }

    // ----------------------------------------------------------------
    // 子ノードを追加
    // ----------------------------------------------------------------
    fn add_children(&mut self, id: &Uuid) -> Result<()> {
        let children: Vec<(PathBuf, NodeType)> = self.find_children(id)?;
        let parent_rank: usize = self.get_rank(id)?;

        // 子ノードをnode_mapにいれる
        let mut children_ids: Vec<Uuid> = Vec::new();
        for (_path, _file_type) in children.iter() {
            let _id = Uuid::new_v4();
            let _child = Node::new(
                _id.clone(),
                _path.clone(), 
                _file_type.clone(), 
                parent_rank + 1
            )?;

            self.node_map.insert(_id.clone(), _child);
            children_ids.push(_id)
        }

        // 子ノードのidを親ノードのchildrenプロパティにセット
        self.set_children_ids(id, Some(children_ids))?;

        return Ok(());
    }

    fn find_children(&self, id: &Uuid) -> Result<Vec<(PathBuf, NodeType)>> {

        let parent_path = self.get_path(id)?;
        let mut children: Vec<(PathBuf, NodeType)> = Vec::new();

        for entry in read_dir(parent_path)? {
            let _path = entry?.path().to_path_buf();
            let _file_type = get_file_type(_path.clone())?;

            children.push((_path, _file_type));
        }
        return Ok(children);
    }
    
    // ----------------------------------------------------------------
    // node_mapの更新
    // ルートから順に再帰的に子ノードを更新していく
    // ----------------------------------------------------------------
    pub fn update(&mut self) -> Result<()> {
        self._update(&self.get_root_id())?;
        return Ok(());
    }

    // ----------------------------------------------------------------
    // updateの内部関数
    // 既にある子ノード  子ノードが存在->子ノードを再帰的に更新, 子ノードがファイルシステム上に存在しない->削除
    // 新しい子ノードがある場合は追加
    // ----------------------------------------------------------------
    fn _update(&mut self, id: &Uuid) -> Result<()> {

        // ノードの消去 -------------------------------------
        let mut existing_paths: Vec<PathBuf> = Vec::new();
        let mut existing_ids: Vec<Uuid> = Vec::new(); //存在する子ノードのみが入るVec

        match self.get_children_ids(id)? {
            Some(ids) => {
                for existed_id in ids.iter() {
                    let existed_path = self.get_path(existed_id)?;

                    // 子ノードのパスが存在している場合は子ノードの更新を再帰的に呼び出し
                    if existed_path.exists() {
                        // 子ノードを更新
                        self._update(existed_id)?;
                        existing_paths.push(existed_path);
                        existing_ids.push(existed_id.clone());
                    }
                    // 既にファイルシステム上に存在しない子ノードがある場合はノードを削除
                    else {
                        self.delete_node(existed_id)?;
                    }
                }
            }
            None => return Ok(()),
        }

        // ノードの追加 ------------------------------------
        let parent_rank = self.get_rank(id)?;
        let new_children = self.find_children(id)?;
        let mut new_ids: Vec<Uuid> = Vec::new();

        for (path, file_type) in new_children.iter() {
            // 新しいファイルやディレクトリが作成されている場合 (= 更新元の子ノードのパスのリストに含まれないパスがある場合)
            if existing_paths.contains(&path) == false {
                let _id = Uuid::new_v4();
                let _child = Node::new(
                    _id.clone(),
                    path.to_path_buf(), 
                    file_type.clone(), 
                    parent_rank + 1
                )?;

                self.node_map.insert(_id.clone(), _child);
                new_ids.push(_id);
            }
        }

        existing_ids.extend(new_ids);

        if existing_ids.len() != 0 {
            self.set_children_ids(id, Some(existing_ids))?;
        } else {
            self.set_children_ids(id, None)?;
        }

        return Ok(());
    }

    // ----------------------------------------------------------------
    // 表示のためにnode_mapを並べる
    // ----------------------------------------------------------------
    pub fn serialize(&self) -> Result<Vec<Uuid>> {
        self._serialize(&self.root_id)
    }

    fn _serialize(&self, id: &Uuid) -> Result<Vec<Uuid>> {

        // ノードが開かれていない場合は引数のidを返す
        if self.get_is_open(id)? == false {
            return Ok(vec![id.clone()])
        }

        // ↓ ノードが開かれているとき　↓
        // 子ノードのidを取得して，再帰的に展開
        let children_ids = match self.get_children_ids(id)? {
            Some(mut ids) => {
                ids = self.sort_ids(ids)?;
                ids = self.ignore_ids(ids)?;
                ids
            }
            None => return Ok(vec![id.clone()]),
        };

        let mut buf: Vec<Uuid> = Vec::new();
        for id in children_ids.iter() {
            let grandchildren = self._serialize(&id)?;
            buf.extend(grandchildren);
        }
        buf.insert(0, id.clone());

        return Ok(buf);
    }
    // idのソート
    fn sort_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Uuid>> {
        let mut nodes = ids.iter().map(|id| self.get_node(id)).collect::<Result<Vec<Node>>>()?;
        nodes.sort_by(Node::sort_fn);
        let sorted_id = nodes.iter().map(|node| node.get_id()).collect::<Vec<Uuid>>(); 
        Ok(sorted_id)
    }
    // config.ignoreにマッチするパターンがあれば抜く
    fn ignore_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Uuid>>{

        // 無視するパターンの集合を生成
        let mut builder = GlobSetBuilder::new();
        for pattern in self.config.ignore.iter() {
            builder.add(Glob::new(pattern)?);
        }
        let set = builder.build()?;

        // idの中から適合するファイル名をもつidを排除
        let filtered = ids.into_iter().filter(|id| {
            match self.get_name(id) {
                Ok(name) => {
                    if set.is_match(name) {
                        return false
                    }
                    return true
                }
                Err(_) => return false
            }
        }).collect();

        return Ok(filtered)
    }

    // ----------------------------------------------------------------
    // ゲッター
    // ----------------------------------------------------------------
    fn get_node(&self, id: &Uuid) -> Result<Node> {
        match self.node_map.get(id) {
            Some(node) => return Ok(node.clone()),
            None => return Err(anyhow!("id: {} does not exist", id)),
        }
    }
    pub fn get_root_id(&self) -> Uuid {
        self.root_id.clone()
    }
    // ----------------------------------------------------------------
    pub fn get_name(&self, id: &Uuid) -> Result<String> {
        match self.node_map.get(id) {
            Some(node) => return Ok(node.get_name()),
            None => return Err(anyhow!("id: {} does not exist", id)),
        }
    }
    // ----------------------------------------------------------------
    pub fn get_path(&self, id: &Uuid) -> Result<PathBuf> {
        match self.node_map.get(id) {
            Some(node) => return Ok(node.get_path()),
            None => return Err(anyhow!("id: {} does not exist", id)),
        }
    }
    // ----------------------------------------------------------------
    pub fn get_rank(&self, id: &Uuid) -> Result<usize> {
        match self.node_map.get(id) {
            Some(node) => return Ok(node.get_rank()),
            None => return Err(anyhow!("id: {} does not exist", id)),
        }
    }
    // ----------------------------------------------------------------
    pub fn get_node_type(&self, id: &Uuid) -> Result<NodeType> {
        match self.node_map.get(id) {
            Some(node) => return Ok(node.get_node_type()),
            None => return Err(anyhow!("id: {} does not exist", id)),
        }
    }

    pub fn get_is_open(&self, id: &Uuid) -> Result<bool> {
        match self.node_map.get(id) {
            Some(node) => return Ok(node.get_is_open()),
            None => return Err(anyhow!("id: {} does not exist", id)),
        }
    }

    // ----------------------------------------------------------------
    pub fn get_length(&self) -> usize {
        self.node_map.len()
    }

    fn get_children_ids(&self, id: &Uuid) -> Result<Option<Vec<Uuid>>> {
        match self.node_map.get(id) {
            Some(node) => return Ok(node.get_children_ids()),
            None => return Err(anyhow!("id: {} does not exist", id)),
        }
    }

    // ----------------------------------------------------------------
    fn set_children_ids(&mut self, id: &Uuid, children: Option<Vec<Uuid>>) -> Result<()> {
        match self.node_map.get_mut(id) {
            Some(node) => {
                node.set_children_ids(children);
                return Ok(());
            }
            None => return Err(anyhow!("id: {} does not exist", id)),
        }
    }

    pub fn set_is_open(&mut self, id: &Uuid, is_open: bool) -> Result<()>{
        match self.node_map.get_mut(id) {
            Some(node) => {
                node.set_is_open(is_open);
                return Ok(())
            }
            None => return Err(anyhow!("id: {} does not exist", id)),
        }
    }
}
