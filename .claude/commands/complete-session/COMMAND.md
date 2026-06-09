---
name: complete-session
description: 完整完成 session 的具体执行步骤
allowed-tools: Bash, Read, Write, Edit, Glob, Grep
argument-hint: 无参数
---

完整完成当前 session 的所有工作。按照下列步骤依次执行，不得跳过。

**详细的每个步骤的规范要求和检查清单见 [`references/README.md`](./references/README.md)**

---

## Step 1: 读取 Session 目标

读取 `docs/track/implementation.md` 和对应的 session 文件，了解当前 session 要完成什么。

参考 [`references/README.md`](./references/README.md) 中的 **Step 1** 部分了解需要提取的具体信息。

**输出**：向用户总结当前 session 的目标和范围

---

## Step 2: 实现代码功能

根据 Step 1 了解到的目标，编写 Rust 代码。

参考 [`references/README.md`](./references/README.md) 中的 **Step 2** 部分了解代码规范要求和检查项。

**输出**：编写完成的代码，能通过 `cargo check` 编译

---

## Step 3: 编写测试代码

为新功能添加单元测试。

参考 [`references/README.md`](./references/README.md) 中的 **Step 3** 部分了解测试编写规范。

**输出**：测试代码编写完成，`cargo test` 全部通过

---

## Step 4: 验证代码正确性

运行 Rust 项目的标准质量检查：

1. `cargo fmt --check` - 代码格式检查
2. `cargo clippy -- -D warnings` - 代码质量检查
3. `cargo test` - 单元测试
4. `cargo build` - 完整编译

参考 [`references/README.md`](./references/README.md) 中的 **Step 4** 部分了解四项检查的详细要求。

**输出**：四项检查全部通过 ✓

---

## Step 5: 更新相关文档

更新 session 记录文件和总导航文件。

参考 [`references/README.md`](./references/README.md) 中的 **Step 5** 部分了解需要更新哪些文件和文档规范。

**输出**：所有文档已更新，遵循规范

---

## Step 6: 生成提交信息

收集改动信息、生成规范化提交信息、暂存更改。

参考 [`references/README.md`](./references/README.md) 中的 **Step 6** 部分了解提交信息格式和提交类型决定规则。

**输出**：提交信息已生成，更改已暂存，等待用户手动 commit

---

## 注意事项

### 一般原则

- ❌ **禁止**自动执行 `git commit`，让用户确认后手动提交
- ✅ **必须**所有四项质量检查都通过才能继续
- ✅ **必须**测试全部通过
- ✅ **必须**文档同时更新

### 如遇故障

如果任何步骤失败（如编译错误、测试失败等）：

1. 输出错误信息和修复建议
2. 提示用户修复后重新调用此 Skill
3. 不继续后续步骤

### 参考文档

详细的规范文档和按步骤分类的参考资料见 [`references/README.md`](./references/README.md)
