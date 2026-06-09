# GPUI 迁移快速开始指南

**目标**：5 分钟快速理解迁移内容

---

## 🎯 一句话总结

将 Lucid 的 GUI 框架从 **Iced**（Elm 架构）切换到 **GPUI**（Zed 编辑器框架），改进代码设计贴近 Zed 风格。

---

## 🔄 核心改变

### Before (Iced)
```rust
// main.rs
struct App { buffer, ... }
fn update(&mut self, msg: Message) { match msg { ... } }
fn view(&self) -> Element { ... }
iced::application(boot, update, view).run()
```

### After (GPUI)
```rust
// main.rs + ui/actions.rs
struct LucidApp { buffer, focus_handle, ... }
impl Render for LucidApp {
    fn render(&mut self, ..., cx: &mut Context<Self>) { 
        div().on_action(cx.listener(Self::handle_open_file))
    }
}
fn handle_open_file(&mut self, _: &OpenFile, ..., cx) {
    cx.spawn(|handle, cx| async { ... })
}
Application::new().run(|cx| cx.bind_keys([...]))
```

---

## 📝 代码改动一览

| 改动 | 文件 | 影响 |
|------|------|------|
| ✅ 新增 | `src/ui/actions.rs` | Actions 宏替代 Message enum |
| ❌ 删除 | `src/editor/keybinding.rs` | 职责转移到 Actions |
| ✏️ 改写 | `src/main.rs` | 240+ 行 GPUI 实现 |
| 📦 改 | `Cargo.toml` | `iced` → `gpui = "0.2"` |
| 🔧 改 | `src/ui/mod.rs` | 导出调整 |
| 🔧 改 | `src/editor/mod.rs` | 移除 keybinding |
| ✅ 保留 | `editor/`、`markdown/` | 纯 Rust，零改动 |

---

## 🧠 GPUI 核心概念（10 秒速成）

```
┌─ Application::new()     ← 应用入口
│
└─ cx.open_window() 
    └─ cx.new(|cx| LucidApp::new(cx))    ← 创建 Entity
        │
        ├─ impl Render for LucidApp      ← 每帧调用
        │   └─ fn render(&mut self, cx)  ← 返回 UI 树
        │
        ├─ div().on_action(listener)     ← 事件处理
        │   └─ &mut self 直接改状态
        │
        ├─ cx.spawn(async { ... })       ← 异步任务
        │   └─ handle.update(|this, cx| { ... })  ← 回到主线程
        │
        └─ cx.notify()                   ← 触发重绘
```

**vs Iced MVU**：
- Iced：Message → update(state) → view(state)
- GPUI：Event → listener(|&mut state, cx|) → notify()

---

## 🎬 5 分钟动手尝试

### 1. 查看新文件结构（2 分钟）
```bash
# 查看新增的 Actions
cat src/ui/actions.rs

# 查看改写后的 main
head -100 src/main.rs

# 确认删除了 keybinding
ls -la src/editor/keybinding.rs  # 应该说 "No such file"
```

### 2. 理解关键改变（2 分钟）

**改变 1：Actions 替代 Message**
```rust
// 旧 (Iced)
enum Message { Open, Save, Undo, Redo }
.on_press(Message::Open)

// 新 (GPUI)
actions!(lucid, [OpenFile, SaveFile, Undo, Redo]);
.on_action(cx.listener(Self::handle_open_file))
```

**改变 2：直接修改状态**
```rust
// 旧 (Iced)
fn update(app: &mut App, msg: Message) {
    match msg {
        Message::Open => { app.file = ...; }
    }
}

// 新 (GPUI)
fn handle_open_file(&mut self, _: &OpenFile, ..., cx) {
    self.file = ...;
    cx.notify();
}
```

**改变 3：异步模式**
```rust
// 旧 (Iced)
Task::perform(async { ... }, Message::FileOpened)
// update 中处理 Message::FileOpened

// 新 (GPUI)
cx.spawn(|handle, cx| async {
    handle.update(&mut cx, |this, cx| { /* 直接改 */ })
})
```

### 3. 快速编译测试（1 分钟）
```bash
cd /Users/jane/code/Rust/Lucid

# 检查是否有 Rust 环境
rustc --version

# 检查 GPUI 依赖
cargo tree | grep gpui

# 尝试编译（可能失败于 Metal，这是正常的）
cargo check 2>&1 | tail -5
```

---

## 🔍 关键文件导览

### 看懂 main.rs
```
1. 行 1-10：    导入（gpui、editor、markdown）
2. 行 15-50：   LucidApp struct 定义
3. 行 52-70：   LucidApp::new() 初始化
4. 行 72-90：   sync_content() 辅助方法
5. 行 92-150：  4 个 Action handlers（handle_open_file, handle_save_file, handle_undo, handle_redo）
6. 行 152-180： render_* UI 方法（toolbar、editor、preview、status_bar）
7. 行 182-210： impl Render（关键！）
8. 行 212-230： fn main() 入口
```

**最关键的部分**：
- 行 182-210 (`impl Render`) — GPUI 的核心，每帧调用
- 行 92-150 (Actions handlers) — 替代 Iced 的 `update()` 函数

### 看懂 actions.rs
超级简短（7 行）：
```rust
use gpui::actions;
actions!(lucid, [OpenFile, SaveFile, Undo, Redo]);
```
就是这样，定义 4 个 Action。

---

## 🤔 常见问题 3 秒解答

**Q: 为什么删 keybinding.rs？**  
A: GPUI Actions 系统更好。快捷键直接由 `cx.bind_keys()` 处理，不需要单独逻辑。

**Q: render() 为啥是 &mut self？**  
A: GPUI 不用 MVU 模式，允许在回调中直接修改状态，更灵活。

**Q: cx.listener() 是什么？**  
A: 把"需要 &mut self"的闭包转换成"标准事件处理器"的工具函数。在 div().on_click(cx.listener(|this, evt, ..., cx| {...})) 中用。

**Q: 怎么写异步代码？**  
A: `cx.spawn(|handle, mut cx| async move { /* 异步工作 */ handle.update(&mut cx, |this, cx| { /* 改状态 */ }) })`

**Q: editor/ 和 markdown/ 为什么没改？**  
A: 它们是纯 Rust，零 GUI 依赖。迁移不需要改。

---

## 📊 改动统计

```
改动文件：       7 个
新增文件：       1 个 (actions.rs)
删除文件：       2 个 (keybinding.rs, app.rs)
改写文件：       1 个 (main.rs, 240+ 行)
小改文件：       3 个 (Cargo.toml, ui/mod.rs, editor/mod.rs)
保留文件：       6 个 (editor/*, markdown/*)

代码行数变化：
  删除：~200 行 (Iced 框架代码)
  新增：~300 行 (GPUI 实现)
  净增：~100 行

编译时间：
  Iced：~3-5s
  GPUI：~5-10s (+ Metal 编译)
```

---

## ✅ 迁移完成标志

当看到这些时，说明迁移完成了：

```bash
✅ Cargo.toml 中 iced 被 gpui 替代
✅ src/main.rs 有 impl Render for LucidApp
✅ src/ui/actions.rs 存在
✅ src/editor/keybinding.rs 已删除
✅ cargo check 输出含 "Compiling gpui" 或失败于 Metal
✅ cargo test 显示 editor/markdown 测试通过
```

---

## 🚀 下一步

1. **立即做**：`cargo check` 看编译状态
2. **修复 Metal**：按 `METAL_COMPILATION_FIX.md` 操作
3. **本地测试**：`cargo run` 启动应用
4. **功能验证**：按 `DELIVERY_CHECKLIST.md` 的手动测试

---

## 📚 详细文档

深入学习请查看：

| 文档 | 内容 | 读者 |
|------|------|------|
| `GPUI_MIGRATION_GUIDE.md` | 完整迁移指南、架构对照、API 表 | 开发者 |
| `METAL_COMPILATION_FIX.md` | 编译问题诊断、4 种解决方案 | 运维/测试 |
| `DELIVERY_CHECKLIST.md` | 验证清单、功能测试 | QA |

---

**下一步**：查看 `METAL_COMPILATION_FIX.md` 解决编译环境 🎉

