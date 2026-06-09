# Lucid - Rust Markdown 编辑器 | 项目规则

> 项目工作规则 | 最后更新：2026-06-09

---

## 核心原则

1. **四阶段开发**：底座 → 权限 → 核心 → 工程（见 `docs/intro/plan.md`）
2. **模块独立**：editor/ 纯 Rust，不依赖 iced
3. **文档同步**：代码完成后立即更新相关文档
4. **质量检查**：提交前必须通过 fmt/clippy/test

---

## ✅ 允许做

- ✅ 英文代码、中文注释、显式生命周期
- ✅ 返回 Result，不 unwrap（业务逻辑）
- ✅ 用 Mermaid 画图（不用 ASCII，除目录结构）
- ✅ 更新 docs/ 中的文档
- ✅ 执行 `git add` 添加文件
- ✅ 给出 git commit 信息

---

## ❌ 禁止做

- ❌ panic/unwrap 在业务逻辑
- ❌ editor/ 导入 iced
- ❌ 跳过测试就提交
- ❌ ASCII 艺术字（除目录结构）
- ❌ **执行 `git commit` 命令**（等用户明确说"提交"）
- ❌ 代码完成后不更新文档

---

## 必读文档

| 场景 | 文档 |
|---|---|
| 第一次工作 | `docs/intro/overview.md` |
| 开发工作流 | `docs/tools/cmd.md` |
| 写代码前 | `docs/rules/code.md` |
| 写文档前 | `docs/rules/docs.md` |
| 查看进度 | `docs/track/status.md` |
| 实现细节 | `docs/track/implementation.md` |

---

## 遇到问题？

| 问题 | 查这个 |
|---|---|
| 如何开发 | `docs/tools/cmd.md` |
| 架构不清 | `docs/intro/arch.md` |
| Rust 概念 | `docs/learn/rust.md` |
| 关键决策 | `docs/ref/decisions.md` |

---

**一句话**：遵守规则 → 查文档 → 开始编码
