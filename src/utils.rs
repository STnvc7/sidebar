pub mod path {
    use crate::node::NodeType;
    use anyhow::{anyhow, Result};
    use path_absolutize::Absolutize;
    use std::env;
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
