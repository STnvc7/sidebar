use anyhow::Result;
use duct::cmd;
use std::path::PathBuf;

// ファイルのオープン====================================
// windows
#[cfg(target_os = "windows")]
pub fn open_editor(path: &PathBuf, command: String) -> Result<()> {
    return Ok(());
}

// linux
#[cfg(target_os = "linux")]
pub fn open_editor(path: &PathBuf, command: String) -> Result<()> {}
//=================================================

// 新しいファイルを作成=================================
#[cfg(target_os = "windows")]
pub fn touch(path: &PathBuf) -> Result<()> {
    return Ok(());
}

// 新しいディレクトリを作成==============================
#[cfg(target_os = "windows")]
pub fn mkdir(path: &PathBuf) -> Result<()> {
    return Ok(());
}

// コピー ==========================================
#[cfg(target_os = "windows")]
pub fn cp(from: &PathBuf, to: &PathBuf) -> Result<()> {
    return Ok(());
}

// 移動===========================================
#[cfg(target_os = "windows")]
pub fn mv(from: &PathBuf, to: &PathBuf) -> Result<()> {
    return Ok(());
}

// 名前の変更 ====================================
// #[cfg(target_os = "windows")]
// pub fn rename(from: &PathBuf, to: &PathBuf) -> Result<()> {
//     return Ok(())
// }

// 削除=========================================
#[cfg(target_os = "windows")]
pub fn rm(path: &PathBuf, recursive: bool) -> Result<()> {
    return Ok(());
}

// 環境変数にパスをエクスポート========================
#[cfg(target_os = "windows")]
pub fn export(path: &PathBuf) -> Result<()> {
    return Ok(());
}

// シンボリックリンクを作成============================
#[cfg(target_os = "windows")]
pub fn ln(old: &PathBuf, new: &PathBuf) -> Result<()> {
    return Ok(());
}
