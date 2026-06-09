# Session 3：文件打开/保存功能实现

> 完成日期：2026-06-09 | 进度：+15% → 60%

## 实现目标

✅ 完成了以下功能：

1. **异步文件 I/O 模块** (`src/editor/file.rs`)
   - 集成 tokio 异步读写
   - 完整错误处理（thiserror）
   - 2 个单元测试

2. **iced 异步任务集成** (`src/main.rs`)
   - 升级 `update` 函数返回 `Task<Message>`
   - 使用 `iced::Task::perform` 执行异步操作
   - 从 `iced::run` 升级到 `iced::application` API

3. **系统文件对话框** (rfd crate)
   - 打开文件选择框（.md/.txt 文件过滤）
   - 保存文件选择框

4. **App 状态扩展**
   - 追踪当前打开文件路径
   - 修改指示符（* 标记）
   - 状态栏消息反馈

5. **UI 改进**
   - 工具栏显示文件名 + 修改状态
   - 底部状态栏显示操作结果
   - 窗口标题动态更新

## 核心实现

### 1. 文件操作模块 (src/editor/file.rs)

```rust
#[derive(Debug, Error)]
pub enum FileError {
    #[error("IO 错误：{0}")]
    IoError(#[from] std::io::Error),
}

pub async fn read_file(path: &Path) -> Result<String, FileError>
pub async fn write_file(path: &Path, content: &str) -> Result<(), FileError>
```

**特点**：
- 纯 Rust，不依赖 iced
- 完整的错误传播（使用 thiserror）
- 异步 API（tokio::fs）

### 2. Message 枚举扩展

```rust
pub enum Message {
    EditInput(String),
    FileOpen,
    FileSave,
    FileOpened(Result<(PathBuf, String), String>),  // 打开完成
    FileSaved(Result<(), String>),                   // 保存完成
}
```

### 3. 异步任务处理

使用 `Task::perform` 执行异步 future：

```rust
// 打开文件
Task::perform(
    async {
        let handle = rfd::AsyncFileDialog::new()
            .add_filter("Markdown", &["md", "txt"])
            .pick_file()
            .await;
        // ... 读取文件
    },
    Message::FileOpened,  // 完成时回调此 Message
)
```

**关键点**：
- rfd 异步文件对话框
- tokio::fs 异步读写
- iced 将异步结果映射为 Message

### 4. App 状态管理

```rust
pub struct App {
    buffer: Buffer,
    preview_html: String,
    file_path: Option<PathBuf>,    // 当前文件路径
    is_modified: bool,              // 修改标记
    status_message: String,         // 状态消息
}
```

## 单元测试

### 文件 I/O 测试 (`src/editor/file.rs`)

```rust
#[tokio::test]
async fn test_write_then_read() {
    // 写入临时文件 → 读出 → 验证内容一致
}

#[tokio::test]
async fn test_read_nonexistent() {
    // 读取不存在文件 → 验证返回 Err
}
```

**测试结果**：✓ 2/2 通过

### 完整测试统计

```
总计：15 个单元测试
- Buffer：3 个
- Parser：5 个
- Renderer：5 个
- File I/O：2 个（新增）

结果：✓ 全部通过
```

## 编译验证

```bash
✓ cargo fmt     - 代码格式化完成
✓ cargo clippy  - 无关键警告（仅框架代码未使用警告）
✓ cargo test    - 15/15 测试通过
✓ cargo build   - 调试版本编译成功（3.2s）
```

## 技术亮点

### 1. iced 0.14 异步模式

当前使用 **函数式 API + Task**（而非旧版 Command）：

| 概念 | 说明 |
|------|------|
| `iced::application(boot, update, view)` | 新 API，支持初始 Task |
| `Task::perform(future, mapper)` | 执行异步 future |
| `Task::none()` | 无操作（同步处理） |

### 2. 任务流程

```
FileOpen Message
  ↓
rfd::AsyncFileDialog (选择文件)
  ↓
tokio::fs::read_to_string (读取内容)
  ↓
Message::FileOpened(Ok((path, content)))
  ↓
update 处理结果，更新 App 状态
```

### 3. 依赖协作

| 库 | 功能 |
|----|------|
| iced (0.14 + tokio feature) | 异步任务执行 |
| tokio (full features) | 异步 I/O runtime |
| rfd | 系统原生文件对话框 |
| thiserror | 错误类型定义 |

## 代码统计

| 组件 | 行数 |
|------|------|
| file.rs（含测试、注释） | ~70 行 |
| main.rs 扩展 | ~130 行（相对 update/view） |
| 其他修改 | ~15 行 |
| 总计新增 | ~215 行 |

## 学习亮点

### 1. iced 异步模式理解

- **以前**（0.12）：`Command<Message>` + `impl Command`
- **现在**（0.14）：`Task<Message>` + `Task::perform`
- **关键**：update 函数可返回 `Task` 以发出异步操作

### 2. tokio 在 iced 中的使用

需要 `iced = { version = "0.14", features = ["tokio"] }` 才能让：
- iced 的 executor 使用 tokio runtime
- 异步 future 中可直接调用 `tokio::fs::*`

### 3. Rust 错误处理最佳实践

使用 `thiserror` 的 `#[from]` 自动实现 `From<std::io::Error>`，
使得 `read_to_string()?.unwrap()` 风格变为简洁的 `read_to_string()?`

## 限制与展望

### 当前限制

1. **文件路径只在内存中**
   - 没有持久化最近打开列表
   - 应用重启后路径丢失

2. **防抖尚未实现**
   - 大文件编辑时，每次按键都重新解析 Markdown
   - 预期阶段二改进

3. **选区/光标未实现**
   - `Cursor` 模块仍是骨架
   - 暂时无法精确定位

### 下阶段计划

**Session 4：快捷键系统**
- Ctrl+O：打开文件
- Ctrl+S：保存文件
- Ctrl+Z/Y：撤销/重做（需先实现 history 模块）
- 预期进度：+5% → 65%

**Session 5+：性能优化**
- 防抖渲染（300ms 延迟）
- 增量解析（仅变化部分）
- 缓存 AST
