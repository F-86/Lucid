# 系统架构 | arch.md

> 项目的整体技术架构和模块设计
> 最后更新：2026-06-09

---

## 整体架构

```
应用层 (app.rs)
  ↓
┌─────────────────────────────┐
│ UI 层 (ui/)                 │
│ ├─ 编辑区 (editor_pane)     │
│ ├─ 预览区 (preview_pane)    │
│ └─ 工具栏 (toolbar)         │
└─────────────────────────────┘
  ↓                 ↓
编辑核心           Markdown 解析
(editor/)          (markdown/)
  ↓                 ↓
buffer            parser
history           renderer
cursor
```

---

## 核心模块

### editor/（纯 Rust，不依赖 iced）
- `buffer.rs` - 文本操作（ropey）
- `history.rs` - 撤销/重做栈
- `cursor.rs` - 光标和选区

**特性**：可独立测试，模块独立

### markdown/
- `parser.rs` - pulldown-cmark 集成
- `renderer.rs` - AST → iced Element

### ui/
- `editor_pane.rs` - 左侧编辑
- `preview_pane.rs` - 右侧预览
- `toolbar.rs` - 工具栏

---

## 关键原则

1. **模块独立**：editor/ 纯逻辑，可单独测试
2. **渐进式**：4 阶段避免过度工程化
3. **数据流**：用户操作 → Message → update → view → 显示

---

**相关**：四阶段规划（`docs/intro/plan.md`）
