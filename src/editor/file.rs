// 文件操作模块 - 纯 Rust，支持异步 I/O
use std::path::Path;
use thiserror::Error;

/// 文件操作的错误类型
#[derive(Debug, Error)]
pub enum FileError {
    #[error("IO 错误：{0}")]
    IoError(#[from] std::io::Error),
}

/// 读取文件内容
///
/// # 参数
/// - `path`：文件路径
///
/// # 返回值
/// 成功返回文件内容字符串，失败返回 FileError
pub async fn read_file(path: &Path) -> Result<String, FileError> {
    let content = tokio::fs::read_to_string(path).await?;
    Ok(content)
}

/// 写入内容到文件
///
/// # 参数
/// - `path`：文件路径
/// - `content`：要写入的内容
///
/// # 返回值
/// 成功返回 ()，失败返回 FileError
pub async fn write_file(path: &Path, content: &str) -> Result<(), FileError> {
    tokio::fs::write(path, content).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// 测试：写入文件后读出，验证内容一致
    #[tokio::test]
    async fn test_write_then_read() {
        let test_dir = std::env::temp_dir();
        let test_file = test_dir.join("lucid_test_write_read.md");

        let original_content = "# 测试\n\n这是一个测试文件。";

        // 写入
        write_file(&test_file, original_content)
            .await
            .expect("写入失败");

        // 读取
        let read_content = read_file(&test_file).await.expect("读取失败");

        // 验证
        assert_eq!(original_content, read_content);

        // 清理
        let _ = std::fs::remove_file(&test_file);
    }

    /// 测试：读取不存在的文件返回错误
    #[tokio::test]
    async fn test_read_nonexistent() {
        let nonexistent_path = PathBuf::from("/tmp/lucid_nonexistent_12345.md");
        let result = read_file(&nonexistent_path).await;

        assert!(result.is_err(), "应该返回错误");
    }
}
