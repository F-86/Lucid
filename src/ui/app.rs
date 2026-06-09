// UI 模块 - Message 定义
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    EditInput(String),
    FileOpen,
    FileSave,
    /// 文件打开完成：(文件路径, 内容) 或错误信息
    FileOpened(Result<(PathBuf, String), String>),
    /// 文件保存完成或错误信息
    FileSaved(Result<(), String>),
}
