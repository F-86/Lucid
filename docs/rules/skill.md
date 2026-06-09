# Skill 规范 | skill.md

> 创建 Skill 的架构规范和最佳实践
> 最后更新：2026-06-09

---

## 核心设计原则

### 职责分离
- **Skill** = 触发条件 + 参数校验（快速响应）
- **Command** = 真实操作执行（实际逻辑）
- 不混合：Skill 中不应有业务逻辑，只有指挥调度

### 响应模式
- **前置检查** → 最开始（跳过无效 Session）
- **后置提示** → 最后（给用户提交信息，不执行 git commit）
- **静默执行** → 中间（无冗余日志）

### 输出规范
- ✅ 最终只输出提交信息
- ✅ 参考 `docs/rules/commits.md` 的格式
- ✅ 所有说明基于规范文档

---

## Skill 架构模式

### 完整结构

Skill 和 Command 采用 markdown 文件格式。一个 Skill 可以对应一个 Command，也可以没有（导航型 Skill）。复杂业务逻辑使用 REFERENCES 索引来组织详细文档：

```
.claude/
├── skills/
│   └── {skill-name}/
│       └── SKILL.md                 # Skill 定义：触发条件 + 前置检查 + 步骤列表
├── commands/
│   └── {command-name}/
│       ├── COMMAND.md               # Command 定义：执行步骤 + 说明
│       └── references/
│           └── README.md            # 可选：按步骤分类的详细规范索引
└── scripts/
    └── {script-name}.sh             # 可选：实际执行的脚本（可以用其他语言）
```

**设计优势**：
- 🎯 导航型 Skill 只需 SKILL.md（如 implementation-guide）
- 🚀 执行型 Skill 配套 COMMAND.md + references（如 complete-session）
- 📚 REFERENCES 模式清晰分离"给用户看"和"给 Claude 看"的文档

### 三个文件的职责

| 文件 | 格式 | 职责 | 必需 | 说明 |
|-----|------|------|-----|-----|
| `.claude/skills/{name}/SKILL.md` | Markdown | 触发条件、前置检查、步骤列表 | ✅ | Skill 的核心定义，确定何时触发和做什么 |
| `.claude/commands/{name}/COMMAND.md` | Markdown | 执行步骤、输出说明、注意事项 | ❌ | 只在 Skill 需要 Command 执行时需要 |
| `.claude/commands/{name}/references/README.md` | Markdown | 按步骤分类的详细规范 | ❌ | 当 COMMAND 步骤众多时，用于组织细节文档 |
| `.claude/scripts/{name}.sh` | Bash | 实际执行的脚本 | ❌ | 可选，可使用其他语言实现 |

### 文件间的关系

```
导航型 Skill (无 Command)
  └─ SKILL.md
       ├─ 说明用途
       ├─ 列出步骤
       └─ 指向相关文档

执行型 Skill (有 Command)
  └─ SKILL.md
       ├─ 前置检查
       ├─ 触发命令 → /command-name
       │
       └─ COMMAND.md
            ├─ Step 1-N 执行步骤
            └─ references/README.md（可选）
                 ├─ Step 1 详细规范
                 ├─ Step 2 详细规范
                 └─ ...
```

**关键设计**：
- **导航型 Skill**：帮助用户理解工作流，指向文档（如 `implementation-guide`）
- **执行型 Skill**：触发 Claude 执行操作，调用 Command（如 `complete-session`）

---

## Skill 的两种模式

### 模式 1：执行型 Skill（完成具体工作）

**目的**：Claude 执行完整工作流（编码、测试、提交等）

**结构**：
- SKILL.md：定义触发、前置检查、步骤列表
- COMMAND.md：详细执行步骤
- references/README.md（可选）：复杂步骤的规范索引

**示例**：`complete-session` Skill
- 用户说："完成当前 session"
- Skill 检查前置条件
- Skill 调用 /complete-session 命令
- Command 执行 6 个步骤，返回结果

**输出**：有形的成果（代码、文档、提交信息等）

---

### 模式 2：导航型 Skill（指导用户理解流程）

**目的**：帮助用户理解复杂的工作流程和文档结构

**结构**：
- SKILL.md：说明文档整体结构、关键概念、快速导航
- 无 COMMAND.md（不调用命令）
- 通过链接指向相关文档

**示例**：`implementation-guide` Skill
- 用户说："如何理解 session 工作流"
- Skill 输出导航信息
- 解释各个文档的用途
- 给出关键文档的链接

**输出**：理解和指导（不执行操作）

---

## Skill 文件格式规范

### Frontmatter 必需字段

```yaml
---
name: skill-name                    # Skill 的唯一标识符，必需
description: 一句话说明 Skill 功能   # Skill 的描述，必需
---
```

**name 字段要求**：
- 必填字段
- 小写英文 + 连字符（如 `complete-session`、`implementation-guide`）
- 与文件夹名称一致
- 用作触发命令：`/complete-session`

### 执行型 Skill 标准格式（SKILL.md）

```markdown
---
name: command-name
description: 一句话说明 Skill 的目的和触发方式
---

TRIGGER when: 
- 用户表述 1
- 用户表述 2
- 用户表述 3

执行前检查（失败则停止）：

1. 检查项 1：描述 + 如何检查的方法
   ```bash
   检查命令
   ```
   
2. 检查项 2：描述 + 如何检查的方法

步骤列表：

Step 1: 步骤简述
Step 2: 步骤简述
Step 3: 步骤简述

执行命令：

\`\`\`
/command-name
\`\`\`

## 工作流说明

这个 Skill [用 1-2 句话说明做什么]

1. [高层次的执行流程说明]
2. [预期结果说明]

### 预期输出

✅ 成功完成会输出：
- [输出 1]
- [输出 2]

❌ 如遇失败会输出：
- [失败类型 1]
- [失败类型 2]

## 注意事项

- [注意 1]
- [注意 2]
```

### 导航型 Skill 标准格式（SKILL.md）

```markdown
---
name: navigation-skill-name
description: 指导用户理解 session 工作流和相关文档结构
---

TRIGGER when:
- 用户想了解 session 如何进行
- 怎么完成一个 session
- 有哪些相关文档

本 Skill 帮助理解：

## 概览

[Skill 的整体目标和结构说明]

## 工作流程

[用 3-5 个关键步骤说明]

## 关键概念

| 概念 | 说明 |
|-----|------|
| 概念 1 | 说明 |

## 相关文档导航

- **文档 1** - [用途](../path/docs.md)
- **文档 2** - [用途](../path/docs.md)

## 快速命令

如果要执行任务而不仅仅是理解，可以使用：

- `/complete-session` - 完整完成 session 的所有工作

详见 [Skill 规范](../rules/skill.md)
```

### 步骤格式要求

| 要求 | 说明 | 例子 |
|-----|------|-----|
| 格式 | `Step N: xxxxx` | `Step 1: 读取 Session 目标` |
| N | 阿拉伯数字，从 1 开始 | 1, 2, 3, 4... |
| xxxxx | 步骤简述，5-10 字 | 不超过一行，简洁明快 |
| 视角 | **给用户看**的高层视角 | 不涉及具体代码细节 |
| 位置 | Skill 中的"步骤列表"部分 | 在执行命令之前 |

**关键差异**：Skill 步骤 vs Command 步骤
- **Skill 步骤**：高层概括（给用户理解工作流）
- **Command 步骤**：详细操作（给 Claude 执行）

---

## REFERENCES 索引设计

### 何时使用 REFERENCES

当 Command 执行步骤众多或每个步骤包含复杂规范时，建议创建 `references/README.md` 来组织详细文档：

```
commands/complete-session/
├── COMMAND.md             # 高层步骤框架（简洁）
└── references/
    └── README.md          # 每个步骤的详细规范
```

**优势**：
- COMMAND.md 保持简洁（～100 行），只列出步骤和重要的说明
- references/README.md 包含所有细节，供 Claude 参考
- 清晰的信息分层：用户看 Skill → Claude 看 Command 框架 → 遇到细节查 references

### REFERENCES 文档结构

```markdown
# Command 参考文档索引

本文档按执行步骤分类，为 Command 执行提供详细规范。

---

### Step 1: 步骤标题

**参考文档**：
- [`docs/path/file.md`](../../../docs/path/file.md) - 说明
- [`docs/path/file2.md`](../../../docs/path/file2.md) - 说明

**必须检查项**：

| 检查项 | 要求 |
|-------|------|
| 项目 1 | 说明 |

**关键规范**：
- ✅ 要求 1
- ❌ 禁止 1

---

### Step 2: ...
```

**关键设计原则**：
- 每个 Step 一个独立的 `###` 章节
- 保持相同的组织结构：参考文档 → 检查项 → 关键规范
- 提供可执行的命令示例
- 从项目特定文档链接到通用规范

---

## 使用示例

### 示例 A：执行型 Skill + 导航型 Skill 组合

**执行型 Skill**：`complete-session` 
- 触发：用户说"完成当前 session"
- 执行：6 个步骤（读取目标 → 实现 → 测试 → 验证 → 文档 → 提交）
- 返回：格式化的提交信息

**导航型 Skill**：`implementation-guide`
- 触发：用户说"如何理解 session 工作流"
- 作用：解释 session 的整个过程
- 指向：`complete-session` Skill 和相关文档

用户可以先用导航型 Skill 了解流程，再用执行型 Skill 完成任务。

---

### 场景 B：Session 进行中（正常执行）

用户说："完成这个 session"

**Skill 前置检查：**
- 检查 git 是否有未提交的更改 → 有
- 检查工作目录是否可用 → 可用
- 结论：Session 进行中，继续执行

**Skill 调用 Command：**
- Command 运行 cargo fmt/clippy/test/build
- 所有检查都通过 ✓
- Command 收集改动文件
- Command 生成提交信息

**Skill 输出：**
```
✓ 质量检查通过
  - cargo fmt ✓
  - cargo clippy ✓
  - cargo test ✓ (11/11 通过)
  - cargo build ✓

📝 生成提交信息：

feat: 完成 session-3 任务

## 核心功能
- 实现文件 I/O 模块
- 添加异步文件操作
- 集成系统文件对话框

## 文件变更
新增：2 个文件
修改：3 个文件

## 质量指标
✓ cargo fmt ✓
✓ cargo clippy ✓
✓ cargo test ✓ (11/11 通过)
✓ cargo build ✓

进度：45% → 60%


📝 下一步：
1. 检查上面的提交信息是否正确
2. 运行命令提交：git commit
   （在编辑器中粘贴上面的提交信息）
3. 验证提交：git show HEAD
```

---

### 场景 C：质量检查失败（任务中止）

用户说："完成当前 session"

**Skill 前置检查：** 通过
**Skill 调用 Command：**
- Command 运行 cargo fmt → ✗ 代码格式不符合
- Command 返回错误

**Skill 输出：**
```
✗ 质量检查失败，无法生成提交信息

❌ 错误详情：
- cargo fmt: 代码格式不符合规范
  修复：运行 cargo fmt 自动格式化

后续操作：
1. 手动运行 cargo fmt
2. 重新调用此 Skill
```

---

## 对比其他设计

### ❌ 常见的错误设计

| 错误 | 原因 | 正确做法 |
|-----|------|--------|
| Command 中输出提示信息 | 职责混淆，提示应该由 Skill 处理 | Command 只返回数据，Skill 处理输出 |
| Skill 自动执行 git commit | 用户无法检查提交信息 | 生成提交信息后让用户手动提交 |
| 不做前置检查 | Session 已完成还在运行，浪费时间 | 前置检查提前判断，有效退出 |
| Command 中 panic/unwrap | 程序会崩溃，用户体验差 | 使用 Result，返回错误信息 |
| 提交信息格式不统一 | 项目历史混乱，难以追溯 | 严格遵循 commits.md 格式 |

---

## 检查清单

### 设计 Skill 前

- [ ] Skill 名称是否清晰且简短？
- [ ] 触发条件是否符合用户习惯？（用户会说什么？）
- [ ] 前置检查逻辑是否清晰？
- [ ] 是否明确参考了对应的规范文档？

### 实现 Command 前

- [ ] Command 职责是否单一？（只执行操作，不输出提示）
- [ ] 是否处理了所有可能的错误？
- [ ] 是否使用 Result 而不是 panic？
- [ ] 返回的数据格式是否清晰？

### 集成 Skill 和 Command 前

- [ ] Skill 是否正确调用了 Command？
- [ ] Command 返回的数据是否被正确处理？
- [ ] 输出信息是否符合 commits.md 格式？

### 测试 Skill 前

- [ ] 前置检查能否提前退出？
- [ ] Command 执行结果是否符合预期？
- [ ] 错误处理是否生效？
- [ ] 最终输出信息是否清晰、准确、有用？

---

## 常见问题

### Q: 为什么不自动执行 git commit？
**A:** 用户需要审查提交信息，确保准确无误。自动提交会跳过这个重要步骤，可能导致错误的提交被推送。

### Q: 前置检查应该检查什么？
**A:** 检查 Session 是否真的需要执行。最常见的是：
- git 有无未提交的更改（没有 → Session 已完成）
- 工作目录是否可用（不可用 → 无法执行）

### Q: Command 应该返回什么？
**A:** 返回完整的提交信息，以及质量检查的结果。Skill 会将这些信息格式化后输出给用户。

### Q: 如果质量检查失败怎么办？
**A:** Command 返回错误信息，Skill 输出错误提示，告诉用户需要修复什么。用户修复后可以重新调用 Skill。

### Q: name 字段有什么用？
**A:** name 字段是 Skill 的唯一标识符，用作触发命令的名称。例如 `name: complete-session` 对应命令 `/complete-session`。

---

## 参考资源

| 文档 | 用途 |
|-----|------|
| `docs/rules/commits.md` | 提交信息格式规范 |
| `docs/rules/code.md` | 代码质量检查标准 |
| `docs/rules/testing.md` | 测试要求 |
| `.claude/CLAUDE.md` | 项目核心规则 |

---

**最后更新**：2026-06-09
