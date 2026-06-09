// 文本缓冲区 - 使用 ropey
use std::result;

#[allow(dead_code)]
pub type Result<T> = result::Result<T, BufferError>;

#[allow(dead_code)]
#[derive(Debug)]
pub enum BufferError {
    InvalidPosition,
    IoError(String),
}

/// 文本缓冲区 - 管理编辑器的文本内容
pub struct Buffer {
    content: String,
}

impl Buffer {
    /// 创建一个新的空缓冲区
    pub fn new() -> Self {
        Buffer {
            content: String::new(),
        }
    }

    /// 获取缓冲区内容
    pub fn content(&self) -> &str {
        &self.content
    }

    /// 设置缓冲区内容
    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
    }

    /// 清空缓冲区
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.content.clear();
    }

    /// 获取行数
    #[allow(dead_code)]
    pub fn line_count(&self) -> usize {
        self.content.lines().count()
    }

    /// 获取字符数
    #[allow(dead_code)]
    pub fn char_count(&self) -> usize {
        self.content.chars().count()
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let buffer = Buffer::new();
        assert_eq!(buffer.content(), "");
    }

    #[test]
    fn test_set_content() {
        let mut buffer = Buffer::new();
        buffer.set_content("# Hello");
        assert_eq!(buffer.content(), "# Hello");
    }

    #[test]
    fn test_line_count() {
        let mut buffer = Buffer::new();
        buffer.set_content("line1\nline2\nline3");
        assert_eq!(buffer.line_count(), 3);
    }
}
