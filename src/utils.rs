pub mod path {
    use crate::node::NodeType;
    use anyhow::{anyhow, Result};
    use path_absolutize::Absolutize;
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};

    pub fn get_cwd_path() -> Result<PathBuf> {
        let cwd = env::current_dir()?;
        return Ok(cwd);
    }

    pub fn resolve_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
        let path = path.as_ref();

        // 相対パスの場合は絶対パスに変換して返す
        if path.is_relative() == true {
            let absolute_path = path.absolutize()?.into_owned();
            return Ok(absolute_path);
        }

        if path.is_symlink() == true {
            let source_path = path.read_link()?;
            return Ok(source_path);
        }

        return Ok(path.to_path_buf());
    }

    pub fn has_child<P: AsRef<Path>>(path: P) -> bool {
        let path = path.as_ref();

        if path.is_dir() {
            // ディレクトリの内容を取得し、最初の要素を確認
            let mut entries = fs::read_dir(path).unwrap();
            return entries.next().is_some();
        } else {
            // 指定されたパスがディレクトリでない場合はエラー
            return false;
        }
    }

    pub fn get_file_type<P: AsRef<Path>>(path: P) -> Result<NodeType> {
        let resolved_path = resolve_path(path)?;

        if resolved_path.is_dir() == true {
            return Ok(NodeType::Folder);
        } else if resolved_path.is_file() == true {
            return Ok(NodeType::File);
        }

        return Err(anyhow!("unable to get file type"));
    }
}
