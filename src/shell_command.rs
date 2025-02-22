#![allow(dead_code)]

use anyhow::Result;
use duct::cmd;
use std::path::PathBuf;
use std::fs;
use log;

// ファイルのオープン====================================
pub fn open_file(path: &PathBuf, command: &String) -> Result<()> {
    cmd!(command, path).stderr_capture().run()?;
    return Ok(());
}

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
// #[cfg(target_os = "windows")]
pub fn mv(from: &PathBuf, to: &PathBuf) -> Result<()> {
    log::info!("{:?}, {:?}", from, to);
    // cmd!("move", from , to).read()?;
    fs::rename(from, to)?;
    Ok(())
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
