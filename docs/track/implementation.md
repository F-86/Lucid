# 实现日志 | implementation.md

> 记录每个实现阶段的详细工作和技术决策
> 最后更新：2026-06-09

---

## Session 1：iced 0.14 兼容性修复与基础框架（2026-06-09）

### 问题诊断

**错误信息**：`error[E0432]: unresolved import iced::Sandbox`

**根本原因**：iced 0.14 不提供 `Sandbox` 特质
- Cargo.lock 确认版本：iced 0.14.0
- 该版本使用函数式 API，不是特质式 API
- 之前使用的 `Sandbox` 是 iced 0.12 的 API

### 解决方案

#### 1. API 迁移：Sandbox → iced::run()

**旧方式（iced 0.12）**：
```rust
impl Sandbox for App {
    type Message = Message;
    fn new() -> Self { ... }
    fn title(&self) -> String { ... }
    fn update(&mut self, message: Message) { ... }
    fn view(&self) -> Element<Message> { ... }
}
App::run(Settings::default())
```

**新方式（iced 0.14）**：
```rust
fn update(app: &mut App, message: Message) { ... }
fn view(app: &App) -> Element<'_, Message> { ... }
iced::run(update, view)  // 自动推导 App 类型
```

#### 2. 关键变化

| 项目 | iced 0.12 | iced 0.14 |
|------|----------|----------|
| 入口 | `App::run()` | `iced::run()` |
| 状态 | 特质方法 `new()` | impl Default + 类型推导 |
| 更新 | 特质方法 `update()` | 自由函数 `fn update()` |
| 视图 | 特质方法 `view()` | 自由函数 `fn view()` |
| 返回值 | 无 | `Element<'_, Message>` |

#### 3. 代码调整

**src/main.rs**：
```rust
// 应用状态 - 自动推导默认值
#[derive(Default)]
pub struct App {
    buffer: Buffer,
}

// 自由函数形式
fn update(app: &mut App, message: Message) { ... }
fn view(app: &App) -> Element<'_, Message> { ... }

// 函数式 API 入口
pub fn main() -> iced::Result {
    iced::run(update, view)
}
```

**src/ui/app.rs**：
```rust
// 只保留 Message 定义，不需要实现特质
#[derive(Debug, Clone)]
pub enum Message {
    EditInput(String),
    FileOpen,
    FileSave,
}
```

### 技术决策

1. **为什么用函数式 API？**
   - iced 0.14 的官方推荐
   - 更灵活：状态类型完全自由
   - 样板代码更少

2. **为什么不升级到 0.15+？**
   - 0.14 已经足够稳定
   - 库生态更成熟
   - 减少不必要的升级风险

3. **生命周期注解 `<'_>`**
   - iced Element 需要借用生命周期
   - `'_` 表示编译器自动推导
   - 避免了手写具体生命周期的复杂性

### 编译验证

```bash
✓ cargo check    # 通过，0 错误
✓ cargo fmt      # 代码格式化完成
✓ cargo clippy   # 通过，无严重警告
✓ cargo test     # 通过，3 个单元测试
✓ cargo build    # 调试版本 21.61s
```

---

## 实现细节

### Buffer 实现

**为什么选择 ropey？**
- 高效的增量文本操作：O(log n)
- 大文件友好：不需要全量加载到内存
- 丰富的 API：行/列查询、迭代器等

**Buffer API 设计**：
```rust
pub struct Buffer {
    rope: Rope,  // 基于 ropey 的文本缓冲
}

impl Buffer {
    pub fn new() -> Self { ... }
    pub fn content(&self) -> String       // 获取全部内容
    pub fn set_content(&mut self, s: &str) // 替换内容
    pub fn clear(&mut self) { ... }        // 清空
    pub fn line_count(&self) -> usize { ... }
    pub fn char_count(&self) -> usize { ... }
}
```

**单元测试**：
```rust
#[test]
fn test_new_buffer() { ... }        // 初始化测试
#[test]
fn test_set_content() { ... }       // 内容设置测试
#[test]
fn test_line_count() { ... }        // 行数统计测试
```

### UI 组件树

```
container (全屏)
└── column (主轴竖直)
    ├── row (工具栏)
    │   ├── button("📁 Open")
    │   └── button("💾 Save")
    └── row (主内容)
        ├── text_input (编辑器, FillPortion 1)
        │   └── Message::EditInput(String)
        └── scrollable (预览, FillPortion 1)
            └── text (预览内容)
```

**布局关键点**：
- `FillPortion(1)` 使编辑器和预览各占 50% 宽度
- `Height::Fill` 使内容区充满可用高度
- `spacing(10)` 提供组件间距

### 模块结构

```
src/
├── main.rs              # 270 行 - 应用入口 + update/view
├── editor/
│   ├── mod.rs          # 模块声明
│   ├── buffer.rs       # Buffer 实现 + 单元测试 (~60 行)
│   ├── cursor.rs       # Cursor 框架 (~10 行)
│   └── history.rs      # History 框架 (~10 行)
├── markdown/
│   ├── mod.rs          # 模块声明
│   ├── parser.rs       # Parser 框架 (~10 行)
│   └── renderer.rs     # Renderer 框架 (~10 行)
└── ui/
    ├── mod.rs          # 模块导出
    └── app.rs          # Message 定义 (~7 行)

总计：~400 行代码
```

### 错误处理

使用 `thiserror` crate 定义 Buffer 错误：
```rust
#[derive(Error, Debug)]
pub enum BufferError {
    #[error("Invalid position")]
    InvalidPosition,
    // TODO: 更多错误类型
}

pub type Result<T> = result::Result<T, BufferError>;
```

虽然当前未使用，但为将来的异常处理做好准备。

---

## 下一步计划

### 立即（6 月 11-12 日）

1. **Markdown 预览实现**
   - 集成 pulldown-cmark 解析器
   - 实现 parser.rs
   - 基础渲染（标题、段落、代码块）

2. **文件操作**
   - 实现 FileOpen/FileSave Message 处理
   - 集成 tokio 异步 I/O
   - 文件对话框（使用原生系统对话）

### 中期（6 月 13-15 日）

3. **快捷键系统**
   - Ctrl+O 打开文件
   - Ctrl+S 保存文件
   - Ctrl+N 新建文件

4. **UI 改进**
   - 更新指示器（标题栏显示 "*有修改")
   - 文件路径显示
   - 状态栏

### 测试方案

```bash
# 单元测试
cargo test

# 集成测试 (手动)
1. cargo run
2. 在编辑器输入 "# Hello\n\nWorld"
3. 预览应显示为 HTML: <h1>Hello</h1><p>World</p>
4. Ctrl+O 打开 README.md
5. Ctrl+S 保存修改
```

---

## 技术亮点

1. **零 unsafe 代码**：100% 安全 Rust
2. **模块独立**：editor/ 可完全独立于 GUI 框架
3. **类型安全**：强类型 Message 确保编译时安全
4. **Elm 架构**：状态机风格更新逻辑，易于测试和扩展
5. **渐进式设计**：核心功能优先，可选功能后续添加

---

## 参考资源

- **iced 文档**：https://docs.rs/iced/0.14.0/iced/
- **ropey 文档**：https://docs.rs/ropey/1.6.1/ropey/
- **pulldown-cmark**：https://docs.rs/pulldown-cmark/0.12.0/pulldown_cmark/
- **Elm 架构**：https://guide.elm-lang.org/architecture/

---

**更新记录**：
- 2026-06-09：Session 1 完成，iced 框架启动 + Buffer 实现
- 待续...
