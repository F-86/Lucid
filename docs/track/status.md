# 项目进度 | status.md

> 实时更新的项目进度追踪
> 最后更新：2026-06-09

---

## 总体进度

```
阶段一（底座）    ▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 60%
阶段二（权限）    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 0%
阶段三（核心）    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 0%
阶段四（工程）    ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 0%

整体：60/100
剩余：5+ 周
```

---

## 当前阶段

**阶段一：最小可运行底座**

状态：实现中  
目标：能启动、编辑、预览  
验证：`cargo run` 后正常工作

✅ 已完成：
- [x] Cargo.toml 初始化 (iced 0.14 修复)
- [x] iced 基础框架 (Application API 适配)
- [x] 双栏布局 (Typora 风格 - 编辑 + 预览)
- [x] 文本编辑 (Buffer 缓冲区 + 输入框)
- [x] 编辑器模块 (buffer/cursor/history 框架)
- [x] Markdown 模块框架 (parser/renderer 待实现)
- [x] **Markdown 解析** (pulldown-cmark 集成) ✓
- [x] **Markdown 渲染** (Parser + Renderer 实现) ✓
- [x] **文件打开/保存** (异步 I/O + rfd 对话框) ✓

待做：
- [ ] 快捷键绑定 (Ctrl+O/S/Z/Y)
- [ ] 状态保存 (最近打开文件)

**进度详细**：

| 组件 | 状态 | 完成日期 |
|------|------|--------|
| iced 框架 | ✅ 完成 | 2026-06-09 |
| 双栏布局 | ✅ 完成 | 2026-06-09 |
| Buffer | ✅ 完成 | 2026-06-09 |
| Message | ✅ 完成 | 2026-06-09 |
| Markdown 解析 | ✅ 完成 | 2026-06-09 |
| Markdown 渲染 | ✅ 完成 | 2026-06-09 |
| Markdown 测试 | ✅ 完成 | 2026-06-09 |
| 文件打开/保存 | ✅ 完成 | 2026-06-09 |
| 快捷键 | ⏳ 待开始 | 预计 2026-06-13 |

---

## 编译状态

✅ **全部通过**

```bash
✓ cargo check   - 编译无错误
✓ cargo fmt     - 代码格式化完成
✓ cargo clippy  - 无严重警告
✓ cargo test    - 13 个单元测试通过（+2 新增 Markdown 测试）
✓ cargo build   - 调试版本构建成功
```

---

## 关键依赖

- iced 0.14 ✓ (已解决 API 兼容性)
- ropey 1.6 ✓ (Buffer 已集成)
- pulldown-cmark 0.12 (Parser 框架就绪)
- thiserror 2
- tokio 1 (异步 I/O 就绪)
- serde 1

---

## 模块结构

```
src/
├── main.rs                 # 应用入口 (update/view 函数)
├── editor/
│   ├── mod.rs
│   ├── buffer.rs          ✓ 文本缓冲 (基于 ropey)
│   ├── cursor.rs          ◐ 光标管理 (框架)
│   └── history.rs         ◐ 撤销/重做 (框架)
├── markdown/
│   ├── mod.rs
│   ├── parser.rs          ✓ Markdown 解析 (pulldown-cmark)
│   └── renderer.rs        ✓ HTML 渲染 + 文本预览
└── ui/
    ├── mod.rs
    └── app.rs             ✓ Message 定义

图例: ✓=完成 ◐=框架就绪
```

---

**详细规划**：`docs/intro/plan.md`  
**架构设计**：`docs/intro/arch.md`  
**代码规范**：`docs/rules/code.md`
