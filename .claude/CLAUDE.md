# Lucid - Rust Markdown 编辑器 | 项目规则

> 精简规则速查表 | 详细文档在 docs/ | 最后更新：2026-06-09

---

## 核心原则

1. **四阶段**：底座 → 权限 → 核心 → 工程（详见 `docs/intro/plan.md`）
2. **模块独立**：editor/ 纯 Rust，不依赖 iced
3. **从外向内**：创建文档先更新上层

---

## 必读文档

| 场景 | 文档 |
|---|---|
| 第一次工作 | `docs/intro/overview.md` |
| 写代码前 | `docs/rules/code.md` |
| 写文档前 | `docs/rules/docs.md` |
| 查询命令 | `docs/tools/cmd.md` |
| 查看进度 | `docs/track/status.md` |

---

## ✅ 允许做

- ✅ 英文代码、中文注释、显式生命周期
- ✅ 返回 Result，不 unwrap（业务逻辑）
- ✅ 用 Mermaid 画图（不用 ASCII，除目录结构）
- ✅ 提交前：`cargo fmt && cargo clippy && cargo test`
- ✅ 执行 `git add` 命令添加文件到暂存区
- ✅ 给出 git commit 信息（用户自己执行 commit）

---

## ❌ 禁止做

- ❌ panic/unwrap 在业务逻辑
- ❌ editor/ 导入 iced
- ❌ 跳过测试就提交
- ❌ ASCII 艺术字（除目录结构）
- ❌ 执行 `git commit` 命令

---

## 遇到问题？

| 问题 | 查这个 |
|---|---|
| 卡住了 | `docs/ref/decisions.md` |
| 架构不清 | `docs/intro/arch.md` |
| Rust 概念 | `docs/learn/rust.md` |

---

**一句话**：遵守规则 → 查文档 → 开始编码
