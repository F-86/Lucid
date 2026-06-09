// 撤销/重做历史栈
// History 用于实现撤销/重做功能

/// 撤销/重做历史栈
/// 维护文本编辑的历史记录，支持撤销（Undo）和重做（Redo）操作
///
/// 数据结构示例：
/// states = [A, B, C, D, E]
/// cursor = 4 (指向 E，当前状态)
///
/// 撤销后：cursor = 3 (返回 D)
/// 重做后：cursor = 4 (返回 E)
/// 编辑后：states = [A, B, C, D, F]，cursor = 4
#[derive(Debug, Clone)]
pub struct History {
    /// 历史状态列表
    states: Vec<String>,
    /// 当前指向的状态索引
    cursor: i32,
    /// 最大历史记录数
    max_entries: usize,
}

impl History {
    /// 创建新的历史栈（最多保存 100 个状态）
    pub fn new() -> Self {
        History {
            states: Vec::new(),
            cursor: -1, // 初始时没有状态
            max_entries: 100,
        }
    }

    /// 记录一个新的编辑状态
    /// 当用户编辑时，调用此方法保存当前状态
    ///
    /// # 参数
    /// * `content` - 当前编辑内容
    pub fn push(&mut self, content: String) {
        let cursor_usize = (self.cursor + 1) as usize;

        // 如果当前位置不在末尾，删除后面的所有状态（新编辑时）
        if cursor_usize < self.states.len() {
            self.states.truncate(cursor_usize);
        }

        // 添加新状态
        self.states.push(content);
        self.cursor = (self.states.len() - 1) as i32;

        // 限制历史记录数量，避免内存溢出
        if self.states.len() > self.max_entries {
            self.states.remove(0);
            self.cursor -= 1;
        }
    }

    /// 撤销一个编辑操作
    /// 返回上一个保存的状态
    pub fn undo(&mut self) -> Option<String> {
        if self.cursor > 0 {
            self.cursor -= 1;
            Some(self.states[self.cursor as usize].clone())
        } else {
            None
        }
    }

    /// 重做一个被撤销的操作
    /// 返回被撤销的状态
    pub fn redo(&mut self) -> Option<String> {
        if self.cursor < (self.states.len() as i32 - 1) {
            self.cursor += 1;
            Some(self.states[self.cursor as usize].clone())
        } else {
            None
        }
    }

    /// 检查是否可以撤销
    #[allow(dead_code)]
    pub fn can_undo(&self) -> bool {
        self.cursor > 0
    }

    /// 检查是否可以重做
    #[allow(dead_code)]
    pub fn can_redo(&self) -> bool {
        self.cursor < (self.states.len() as i32 - 1)
    }

    /// 清空所有历史记录
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.states.clear();
        self.cursor = -1;
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试基本的撤销功能
    #[test]
    fn test_undo_basic() {
        let mut history = History::new();
        history.push("first".to_string());
        history.push("second".to_string());
        history.push("third".to_string());

        // 撤销一步
        let result = history.undo();
        assert_eq!(result, Some("second".to_string()));
        assert!(history.can_undo());
        assert!(history.can_redo());
    }

    /// 测试基本的重做功能
    #[test]
    fn test_redo_basic() {
        let mut history = History::new();
        history.push("first".to_string());
        history.push("second".to_string());
        history.push("third".to_string());

        // 撤销
        history.undo();
        // 重做
        let result = history.redo();
        assert_eq!(result, Some("third".to_string()));
        assert!(!history.can_redo());
    }

    /// 测试新编辑后重做栈被清空
    #[test]
    fn test_new_edit_clears_redo() {
        let mut history = History::new();
        history.push("first".to_string());
        history.push("second".to_string());
        history.push("third".to_string());

        // 撤销
        history.undo();
        assert!(history.can_redo());

        // 新编辑
        history.push("fourth".to_string());
        // 重做栈应被清空
        assert!(!history.can_redo());
        // 当前应该指向 "fourth"，撤销一次应该回到 "second"
        assert_eq!(history.undo(), Some("second".to_string()));
    }

    /// 测试空栈上的撤销返回 None
    #[test]
    fn test_undo_empty_stack() {
        let mut history = History::new();
        let result = history.undo();
        assert_eq!(result, None);
        assert!(!history.can_undo());
    }

    /// 测试空栈上的重做返回 None
    #[test]
    fn test_redo_empty_stack() {
        let mut history = History::new();
        let result = history.redo();
        assert_eq!(result, None);
        assert!(!history.can_redo());
    }

    /// 测试清空历史
    #[test]
    fn test_clear_history() {
        let mut history = History::new();
        history.push("first".to_string());
        history.push("second".to_string());
        history.undo();

        history.clear();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }

    /// 测试多次撤销/重做
    #[test]
    fn test_multiple_undo_redo() {
        let mut history = History::new();
        history.push("A".to_string());
        history.push("B".to_string());
        history.push("C".to_string());

        // 撤销两次
        assert_eq!(history.undo(), Some("B".to_string()));
        assert_eq!(history.undo(), Some("A".to_string()));

        // 重做一次
        assert_eq!(history.redo(), Some("B".to_string()));

        // 新编辑
        history.push("D".to_string());

        // 现在应该只能撤销两次
        assert_eq!(history.undo(), Some("B".to_string()));
        assert_eq!(history.undo(), Some("A".to_string()));
        assert_eq!(history.undo(), None);
    }
}
