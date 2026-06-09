# 工程思路总览 | overview.md

> 用途：第一次工作时了解项目整体思路
> 最后更新：2026-06-09

---

## 项目是什么

**Lucid** - 用 Rust 构建的类 Typora Markdown 编辑器

- 目标：学习 Rust + 做出可用产品
- 时间：8 周（4 阶段）
- 技术：iced + ropey + pulldown-cmark

---

## 为什么这样设计

- **模块独立**：editor/ 纯 Rust，可单独测试
- **循序渐进**：4 阶段避免过度工程化
- **规则先行**：CLAUDE.md 快速查，docs/ 详细看
- **工程化**：第 4 阶段补完测试和文档

---

## 核心依赖

| 库 | 用途 |
|---|---|
| iced | GUI 框架（Elm 架构） |
| ropey | 文本操作（Rope 数据结构） |
| pulldown-cmark | Markdown 解析 |
| thiserror | 错误处理 |
| tokio | 异步运行时 |

---

## 快速链接

- 四阶段详情：`docs/intro/plan.md`
- 系统架构：`docs/intro/arch.md`
- 代码规范：`docs/rules/code.md`
- 今天做什么：`docs/track/todo.md`

---

**下一步**：阅读 `docs/intro/plan.md`
