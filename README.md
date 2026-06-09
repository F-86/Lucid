# Lucid - Rust Markdown 编辑器

> 用 Rust 构建类 Typora 的 Markdown 编辑器 | 学习 Rust 的完整项目

---

## 快速开始

1. 阅读 `.claude/CLAUDE.md`（50 行规则）
2. 查看 `docs/intro/overview.md`（项目概览）
3. 查看 `docs/tools/cmd.md`（开发工作流）
4. 按需查阅其他文档
5. 开始编码

---

## 文档导航

```
docs/
├─ intro/                    ← 入门相关
│  ├─ overview.md            工程思路总览
│  ├─ plan.md                四阶段详细规划
│  └─ arch.md                系统架构设计
├─ rules/                    ← 规范相关
│  ├─ code.md                代码规范
│  └─ docs.md                文档规范
├─ learn/                    ← 学习相关
│  └─ rust.md                Rust 学习路线
├─ track/                    ← 进度相关
│  ├─ status.md              项目进度
│  ├─ implementation.md       实现细节
│  ├─ todo.md                任务清单
│  └─ check.md               完整性检查
├─ tools/                    ← 工具相关
│  └─ cmd.md                 常用命令和工作流
└─ ref/                      ← 参考相关
   └─ decisions.md           关键决策
```

---

## 核心规则

详见 `.claude/CLAUDE.md`：

- ✅ 英文代码、中文注释
- ✅ editor/ 纯 Rust（不依赖 iced）
- ✅ 返回 Result，不 unwrap
- ✅ 用 Mermaid 画图（不用 ASCII）
- ✅ 代码完成后立即更新文档
- ❌ 不提交未测试的代码
- ❌ 不主动执行 git commit

---

## 快速命令

```bash
# 开发工作流
cargo check
cargo test
cargo fmt && cargo clippy

# 详细命令见 docs/tools/cmd.md
```

---

**规则在 CLAUDE.md 📋 | 文档在 docs/ 📚 | 工作流在 cmd.md 🔄**
