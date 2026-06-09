# 系统架构 | arch.md

> 项目的整体技术架构和模块设计
> 最后更新：2026-06-09

---

## 整体架构（Typora 风格）

```
┌─────────────────────────────────────────────┐
│          iced::run(update, view)            │  <- 应用入口
├─────────────────────────────────────────────┤
│  App { buffer: Buffer }                     │  <- 应用状态
│  Message { EditInput, FileOpen, FileSave }  │  <- 用户事件
├─────────────────────────────────────────────┤
│ 工具栏 (toolbar)                            │
│ ┌──────────────────┬──────────────────┐     │
│ │  📁 Open         │  💾 Save         │     │
│ └──────────────────┴──────────────────┘     │
├─────────────────────────────────────────────┤
│         双栏布局（Typora 风格）             │
│ ┌─────────────────┬──────────────────┐      │
│ │   编辑器        │      预览        │      │
│ │ text_input      │   scrollable     │      │
│ │ (FillPortion1)  │   (FillPortion1) │      │
│ │                 │                  │      │
│ │ 实时同步 ←─────────────→            │      │
│ │                 │                  │      │
│ └─────────────────┴──────────────────┘      │
└─────────────────────────────────────────────┘
         ↓                    ↓
    编辑核心            Markdown 解析
    (editor/)           (markdown/)
         ↓                    ↓
    buffer.rs          parser.rs
    cursor.rs          renderer.rs
    history.rs
```

---

## 核心模块实现

### 1. editor/（纯 Rust，不依赖 iced）

#### buffer.rs ✓
- 基于 **ropey** 的高效文本缓冲区
- API:
  ```rust
  pub struct Buffer { rope: Rope }
  pub fn new() -> Self
  pub fn content(&self) -> String       // 获取全部内容
  pub fn set_content(&mut self, s: &str) // 替换内容
  pub fn clear(&mut self)               // 清空内容
  pub fn line_count(&self) -> usize     // 行数
  pub fn char_count(&self) -> usize     // 字符数
  ```
- 单元测试：✓ 3 个

#### cursor.rs ◐
- 框架已创建，待实现
- 计划功能：
  - 光标位置 (line, col)
  - 选区范围 (start, end)
  - 移动操作 (左/右/上/下)

#### history.rs ◐
- 框架已创建，待实现
- 计划功能：
  - 撤销/重做栈
  - Action 记录
  - 撤销/重做命令

### 2. markdown/（Markdown 处理）

#### parser.rs ◐
- 框架已创建
- 计划集成：pulldown-cmark 0.12
- API:
  ```rust
  pub struct Parser;
  pub fn parse(markdown: &str) -> Result<CowStr, String>
  ```

#### renderer.rs ◐
- 框架已创建
- 计划功能：
  - AST → iced Element
  - 支持：粗体、斜体、代码块、列表等
  - 语法高亮集成

### 3. ui/

#### main.rs ✓
- 应用入口和状态管理
- `iced::run(update, view)` 函数式 API
- 自动推导应用状态类型

#### app.rs ✓
- Message 枚举定义
- 3 个消息类型：
  - `EditInput(String)` - 编辑器输入
  - `FileOpen` - 打开文件
  - `FileSave` - 保存文件

#### update 函数 ✓
- 处理 Message → 更新 App 状态
- 当前逻辑：
  - `EditInput` → 更新 Buffer 内容
  - `FileOpen/FileSave` → TODO

#### view 函数 ✓
- 生成 UI Element
- 组件树：
  ```
  container(
    column![
      toolbar(row![button, button]),
      content(row![
        editor(text_input),
        preview(scrollable(text))
      ])
    ]
  )
  ```

---

## 数据流

```
用户操作
   ↓
iced 事件 → Message
   ↓
update(app, message) → 修改 App 状态
   ↓
view(app) → 生成 Element
   ↓
iced 渲染 → 屏幕显示
```

### 当前示例流程

```
用户在编辑器输入 "# Hello"
   ↓
EditInput("# Hello") Message
   ↓
update() → app.buffer.set_content("# Hello")
   ↓
view() 重新渲染
   ├─ editor → text_input 显示 "# Hello"
   └─ preview → text 显示 "预览\n# Hello"
   ↓
屏幕更新
```

---

## 关键原则

1. **模块独立**：editor/ 纯逻辑，0 iced 依赖，可单独单元测试
2. **Typora 风格**：单框编辑 + 右侧实时预览，不是 VSCode 式分栏
3. **渐进式**：4 阶段开发，避免过度工程化
4. **状态驱动**：Elm 架构风格（update → view）
5. **异步就绪**：tokio 集成准备就绪，用于文件 I/O

---

## iced 0.14 特性

- **函数式 API**：`iced::run(update, view)` 而不是特质实现
- **状态类型推导**：从 `view` 的参数类型自动推导
- **macro 简化**：`row![]`、`column![]` 减少样板代码
- **widget 完整**：text_input、scrollable、button 等开箱即用

---

## 编译检查状态

```
✓ cargo check   - 通过（0 错误）
✓ cargo fmt     - 通过（代码格式化）
✓ cargo clippy  - 通过（无严重警告）
✓ cargo test    - 通过（3 个单元测试）
✓ cargo build   - 通过（调试版本）
```

---

**相关文档**：  
- 四阶段规划：`docs/intro/plan.md`
- 代码规范：`docs/rules/code.md`
- 快速开始：`docs/intro/overview.md`
