# 系统架构 | arch.md

> 项目的整体技术架构和模块设计
> 最后更新：2026-06-09

---

## 整体架构（Typora 风格）

```
应用层 (app.rs)
  ↓
┌─────────────────────────────────┐
│ 工具栏 (toolbar)                │
├─────────────────────────────────┤
│  编辑区 + 实时预览（同一个框）   │
│  - 用户在编辑器打字              │
│  - 右侧/下方实时显示预览        │
│  - 滚动同步                      │
└─────────────────────────────────┘
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
- `editor.rs` - 编辑器主体（Typora 样式）
- `preview.rs` - 实时预览面板
- `toolbar.rs` - 工具栏

---

## 关键原则

1. **模块独立**：editor/ 纯逻辑，可单独测试
2. **Typora 风格**：单框编辑 + 实时预览，不是左右分栏
3. **渐进式**：4 阶段避免过度工程化
4. **数据流**：用户操作 → Message → update → view → 显示

---

**相关**：四阶段规划（`docs/intro/plan.md`）
