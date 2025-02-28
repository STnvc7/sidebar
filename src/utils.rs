pub mod path {
    use crate::node::NodeType;
    use anyhow::{anyhow, Result};
    use path_absolutize::Absolutize;
    use std::env;
    use std::path::{Path, PathBuf};
    use std::fs;

    pub fn get_application_root() -> Result<PathBuf> {
        let app_root = match dir::home_dir() {
            Some(path) => path.join(".sidebar"),
            None => return Err(anyhow!("Impossible to get your home dir!"))
        };
        if app_root.exists() == false {
            fs::create_dir_all(&app_root)?;
        }

        return Ok(app_root)
    }

    pub fn get_cwd_path() -> Result<PathBuf> {
        let cwd = env::current_dir()?;
        return Ok(cwd);
    }

    pub fn resolve_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
        let mut path = path.as_ref().to_path_buf();

        // 相対パスの場合は絶対パスに変換
        if path.is_relative() == true {
            path = path.absolutize()?.into_owned();
        }

        // シンボリックリンクの場合は基のパスを取得
        if path.is_symlink() == true {
            path = path.read_link()?;
        }

        return Ok(path);
    }

    pub fn get_file_type<P: AsRef<Path>>(path: P) -> Result<NodeType> {
        let resolved_path = resolve_path(path)?;
        if resolved_path.exists() == false {
            return Ok(NodeType::Unknown)
        }
        
        if resolved_path.is_dir() == true {
            return Ok(NodeType::Folder);
        } else if resolved_path.is_file() == true {
            return Ok(NodeType::File);
        }

        return Err(anyhow!("Impossible to get file type!"));
    }
}
