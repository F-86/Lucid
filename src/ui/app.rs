// UI 模块 - Message 定义
use iced::widget::text_editor;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    /// 编辑器输入事件
    #[allow(dead_code)]
    EditInput(String),
    /// 编辑器操作
    EditorAction(text_editor::Action),
    /// 文件打开请求
    FileOpen,
    /// 文件保存请求
    FileSave,
    /// 文件打开完成：(文件路径, 内容) 或错误信息
    FileOpened(Result<(PathBuf, String), String>),
    /// 文件保存完成或错误信息
    FileSaved(Result<(), String>),
    /// 撤销请求 (Ctrl+Z)
    Undo,
    /// 重做请求 (Ctrl+Y 或 Ctrl+Shift+Z)
    Redo,
}
