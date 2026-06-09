# 提交规范 | commits.md

> Git 提交信息的编写规则
> 最后更新：2026-06-09

---

## 核心原则

1. **一次提交做一件事** - 避免混杂多个功能
2. **清晰的提交历史** - 他人能理解每次改动的目的
3. **可追溯性** - 通过 commit message 快速定位问题
4. **中英文混用** - 标题英文，内容中文

---

## 提交信息格式

```
<类型>: <简短描述>（建议 50 字以内）

<详细说明>（可选，必要时换行说明）

<关闭的 Issue>（可选）
```

### 示例

```
feat: 实现文件打开/保存功能

- 添加 src/editor/file.rs 异步 I/O 模块
- 集成 rfd 系统文件对话框
- 扩展 Message 枚举支持异步结果
- 状态栏显示操作反馈

进度：45% → 60%
测试：15/15 通过 ✓
```

---

## 提交类型（Type）

| 类型 | 说明 | 例子 |
|------|------|------|
| `feat` | 新功能 | feat: 添加快捷键系统 |
| `fix` | 修复 bug | fix: 修复预览滚动位置 |
| `refactor` | 代码重构 | refactor: 简化 Message 处理 |
| `perf` | 性能优化 | perf: 添加防抖渲染 |
| `test` | 添加/修改测试 | test: 添加文件 I/O 测试 |
| `docs` | 文档更新 | docs: 更新项目规范 |
| `chore` | 依赖、构建等 | chore: 升级 iced 到 0.14 |
| `ci` | CI/CD 配置 | ci: 添加 GitHub Actions |

---

## 简短描述（Summary）

### ✅ 好的例子

```
feat: 实现文件打开/保存功能
fix: 修复 Markdown 列表渲染错误
docs: 补完项目规范文档
test: 添加 Buffer 单元测试
```

### ❌ 不好的例子

```
更新代码                          # 过于模糊
implemented file i/o              # 用中文描述
feat: 修复了很多bug               # 不清楚是哪些
```

### 写作建议

- 使用命令式（祈使语气）：「实现」而非「已实现」
- 简洁明了：描述是**什么**，不是**怎么做**
- 不超过 50 字：标题要快速扫过能理解

---

## 详细说明（Body）

### 何时需要

- 功能较复杂需要解释设计
- 包含多个文件的改动
- 有重要的技术决策

### 格式要求

```
<空行>
- 使用 bullet list 列举改动
- 每个 bullet 一行
- 说明**为什么**做这个改动

<空行>
进度：X% → Y%
测试：Z/Z 通过 ✓
```

### 详细说明示例

```
feat: 实现异步文件 I/O

- 创建 src/editor/file.rs：纯 Rust 异步 I/O 模块
- 集成 rfd 系统文件对话框（macOS/Linux/Windows）
- 升级 iced::run → iced::application 支持 Task
- 扩展 App 状态追踪文件路径和修改标记
- 添加 2 个单元测试验证读写功能

## 技术决策
- 使用 Task::perform 而非旧版 Command
- tokio::fs 而非 std::fs（异步不卡顿）
- thiserror 错误处理而非手写 From

进度：45% → 60% (+15%)
测试：15/15 通过（+2 文件 I/O 测试）
质量：fmt ✓ clippy ✓ build ✓
```

---

## 特殊说明标签

### 进度标签

```
进度：45% → 60%           # 更新阶段进度
进度：+15%（总 60%）      # 或者这种格式
```

### 测试标签

```
测试：15/15 通过 ✓       # 单元测试通过
测试：新增 2 个测试      # 测试数量变化
```

### 质量标签

```
质量：fmt ✓ clippy ✓ build ✓
```

### 文件变更总结

```
## 文件变更
新增：
  - src/editor/file.rs (70 行)
  - docs/test/session3-manual-test.md

修改：
  - src/main.rs (130 行)
  - Cargo.toml (+2 依赖)

删除：
  - (如果有的话)
```

---

## 每个 Session 的提交模式

### 单个 Session 提交

```
feat: <Session 标题>

## 核心功能
- <主要功能1>
- <主要功能2>

## 技术方案
- <技术点1>
- <技术点2>

## 文件变更
新增/修改/删除：...

## 质量指标
✓ cargo fmt/clippy/test/build

进度：X% → Y%
```

### 多个相关小改动

如果在同一个 Session 中有多个独立的改动，可以分成多个提交：

```
git commit -m "chore: 添加 rfd 依赖"
git commit -m "feat: 实现文件打开功能"
git commit -m "feat: 实现文件保存功能"
git commit -m "docs: 更新项目规范"
```

但通常建议**一个 Session 一个提交**，这样历史更清晰。

---

## 避免的做法

### ❌ 不清晰的提交

```
git commit -m "update"
git commit -m "fix bugs"
git commit -m "WIP"
```

### ❌ 过长的单行

```
git commit -m "feat: 实现了文件打开保存修改指示符状态栏等一堆功能"
```

→ 用详细说明替代

### ❌ 过多改动混在一起

```
git commit -m "feat: 添加文件 I/O、快捷键、防抖渲染"
```

→ 分成多个提交

### ❌ 使用 -m 跳过编辑器

```
git commit -m "短消息"   # 不推荐，用编辑器写更丰富的信息
```

→ 用 `git commit`（无 -m）打开编辑器

---

## 最佳实践

### 1. 提交前检查

```bash
# 查看修改内容
git diff

# 查看要提交的内容
git diff --staged

# 确认无误再提交
git commit
```

### 2. 使用编辑器写提交信息

```bash
# 打开默认编辑器（通常是 vim）
git commit

# 或指定编辑器
EDITOR=nano git commit
```

### 3. 一次提交的理想大小

- **不超过 10 个文件改动**
- **相关功能聚在一起**
- **可单独 revert 而不破坏其他功能**

### 4. 查看提交历史

```bash
# 简洁格式
git log --oneline

# 详细格式
git log -p

# 图形化查看分支
git log --graph --oneline --all
```

---

## 工作流示例

### 正确的工作流

```bash
# 1. 创建功能分支
git checkout -b feature/file-io

# 2. 实现功能、编写测试
cargo build
cargo test
cargo fmt
cargo clippy

# 3. 提交到本地
git add -A
git commit

# 4. 确认提交信息
# 在编辑器中填写详细说明

# 5. 查看提交
git log -1
git show HEAD

# 6. 合并到主分支
git checkout main
git merge --no-ff feature/file-io

# 7. 推送
git push origin main
```

---

## 提交信息检查清单

在每次提交前，问自己：

```
□ 提交类型是否正确？（feat/fix/refactor/...）
□ 简短描述是否清晰？（50 字以内）
□ 是否使用中文描述？（代码英文，描述中文）
□ 详细说明是否充分？（为什么做这个改动）
□ 文件变更是否列举？（新增/修改/删除）
□ 测试是否通过？（cargo test）
□ 质量检查是否通过？（fmt/clippy/build）
□ 进度是否更新？（X% → Y%）
□ Co-Author 是否正确？
```

---

## 参考资源

- [Conventional Commits](https://www.conventionalcommits.org/)
- [How to Write a Git Commit Message](https://chris.beams.io/posts/git-commit/)
- 项目规范：`CLAUDE.md`
- 代码规范：`docs/rules/code.md`
