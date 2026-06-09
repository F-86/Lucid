/// 快捷键绑定模块 - 管理全局快捷键
/// 提供统一的快捷键处理接口
use crate::ui::Message;

/// 快捷键配置
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Keybindings {
    /// Ctrl+O: 打开文件
    pub open: (bool, char),
    /// Ctrl+S: 保存文件
    pub save: (bool, char),
    /// Ctrl+Z: 撤销
    pub undo: (bool, char),
    /// Ctrl+Y: 重做
    pub redo: (bool, char),
}

impl Default for Keybindings {
    fn default() -> Self {
        Keybindings {
            open: (true, 'o'), // Ctrl+O
            save: (true, 's'), // Ctrl+S
            undo: (true, 'z'), // Ctrl+Z
            redo: (true, 'y'), // Ctrl+Y
        }
    }
}

impl Keybindings {
    /// 检查是否按下打开文件快捷键
    #[allow(dead_code)]
    pub fn is_open(&self, ctrl: bool, key: char) -> bool {
        ctrl == self.open.0 && key.to_lowercase().to_string() == self.open.1.to_string()
    }

    /// 检查是否按下保存文件快捷键
    #[allow(dead_code)]
    pub fn is_save(&self, ctrl: bool, key: char) -> bool {
        ctrl == self.save.0 && key.to_lowercase().to_string() == self.save.1.to_string()
    }

    /// 检查是否按下撤销快捷键
    #[allow(dead_code)]
    pub fn is_undo(&self, ctrl: bool, key: char) -> bool {
        ctrl == self.undo.0 && key.to_lowercase().to_string() == self.undo.1.to_string()
    }

    /// 检查是否按下重做快捷键
    #[allow(dead_code)]
    pub fn is_redo(&self, ctrl: bool, key: char) -> bool {
        ctrl == self.redo.0 && key.to_lowercase().to_string() == self.redo.1.to_string()
    }

    /// 根据按键组合返回对应的 Message
    #[allow(dead_code)]
    pub fn get_message(&self, ctrl: bool, shift: bool, key: char) -> Option<Message> {
        if self.is_open(ctrl, key) {
            Some(Message::FileOpen)
        } else if self.is_save(ctrl, key) {
            Some(Message::FileSave)
        } else if self.is_undo(ctrl, key) && !shift {
            Some(Message::Undo)
        } else if self.is_redo(ctrl, key) {
            Some(Message::Redo)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_keybinding() {
        let keys = Keybindings::default();
        assert!(keys.is_open(true, 'o'));
        assert!(!keys.is_open(false, 'o'));
        assert!(!keys.is_open(true, 'p'));
    }

    #[test]
    fn test_save_keybinding() {
        let keys = Keybindings::default();
        assert!(keys.is_save(true, 's'));
        assert!(!keys.is_save(false, 's'));
    }

    #[test]
    fn test_undo_keybinding() {
        let keys = Keybindings::default();
        assert!(keys.is_undo(true, 'z'));
        assert!(!keys.is_undo(false, 'z'));
    }

    #[test]
    fn test_redo_keybinding() {
        let keys = Keybindings::default();
        assert!(keys.is_redo(true, 'y'));
        assert!(!keys.is_redo(false, 'y'));
    }

    #[test]
    fn test_get_message_open() {
        let keys = Keybindings::default();
        assert_eq!(
            format!("{:?}", keys.get_message(true, false, 'o')),
            "Some(FileOpen)"
        );
    }

    #[test]
    fn test_get_message_undo() {
        let keys = Keybindings::default();
        assert_eq!(
            format!("{:?}", keys.get_message(true, false, 'z')),
            "Some(Undo)"
        );
    }
}
