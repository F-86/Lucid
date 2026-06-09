// UI 模块 - Message 定义
#[derive(Debug, Clone)]
pub enum Message {
    EditInput(String),
    FileOpen,
    FileSave,
}
