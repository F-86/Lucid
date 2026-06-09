# Command 参考文档索引

本目录包含 `complete-session` Command 在各个执行步骤中需要参考的规范文档。

## 📋 按步骤分类参考

### Step 1: 读取 Session 目标

**参考文档**：
- [`docs/track/implementation.md`](../../../docs/track/implementation.md) - 了解当前是第几个 session、整体进度
- `docs/track/sessions/session-N.md` - 了解该 session 的详细目标、实现范围、技术方案

**需要提取的信息**：
- Session 标题和编号
- 实现目标列表
- 功能范围和核心实现
- 预期进度增长

---

### Step 2: 实现代码功能

**参考文档**：
- [`docs/rules/code.md`](../../../docs/rules/code.md) - 代码规范和质量标准

**必须检查项**：

| 检查项 | 说明 |
|-------|------|
| 命名规范 | 代码英文，注释中文 |
| 函数签名 | rustdoc 文档 + 显式生命周期 |
| 错误处理 | 返回 Result，不使用 unwrap |
| 模块独立 | editor/ 不导入 iced |
| 编译通过 | `cargo check` 无错误 |

**关键规范**：
- ✅ 英文代码、中文注释
- ✅ 返回 Result，不 unwrap
- ✅ 编译前后 `cargo fmt && cargo clippy`
- ❌ 禁止 panic/unwrap 在业务逻辑

---

### Step 3: 编写测试代码

**参考文档**：
- [`docs/rules/testing.md`](../../../docs/rules/testing.md) - 测试编写规范

**必须检查项**：

| 检查项 | 要求 |
|-------|------|
| 命名规范 | `test_{功能}_{场景}_{预期}` |
| 覆盖范围 | 成功路径 + 失败路径 + 边界情况 |
| 测试位置 | 在模块末尾 `#[cfg(test)]` |
| 注释说明 | 说明测试意图 |
| 无依赖 | 不依赖文件系统等外部状态 |

**关键规范**：
- ✅ 每个公开函数都应有测试
- ✅ 验证成功和失败的路径
- ✅ 测试应独立且可重复执行
- ❌ 禁止使用 unwrap（测试中也不行）

**运行测试**：
```bash
cargo test
# 输出应显示：test result: ok. N passed
```

---

### Step 4: 验证代码正确性

**参考文档**：
- [`docs/rules/code.md`](../../../docs/rules/code.md) - 质量检查标准

**四项检查清单**：

1. **代码格式**
   ```bash
   cargo fmt --check
   ```
   如果失败，运行 `cargo fmt` 自动修复

2. **代码质量**
   ```bash
   cargo clippy -- -D warnings
   ```
   必须修复所有警告

3. **单元测试**
   ```bash
   cargo test
   ```
   所有测试必须通过（0 失败）

4. **完整编译**
   ```bash
   cargo build
   ```
   编译成功，无错误

**预期输出**：
```
✓ cargo fmt
✓ cargo clippy
✓ cargo test (N/N 通过)
✓ cargo build
```

---

### Step 5: 更新相关文档

**参考文档**：
- [`docs/rules/docs.md`](../../../docs/rules/docs.md) - 文档规范
- [`docs/track/implementation.md`](../../../docs/track/implementation.md) - 导航结构示例

**需要更新的文件**：

1. **Session 记录** (`docs/track/sessions/session-N.md`)
   - 实现目标 - 完成了什么
   - 核心实现 - 主要组件
   - 单元测试 - 测试统计
   - 编译验证 - fmt/clippy/test/build 结果
   - 代码统计 - 新增行数
   - 技术亮点 - 关键设计
   - 学习亮点 - 学到了什么

2. **总导航更新** (`docs/track/implementation.md`)
   - 整体进度表 - 更新百分比
   - Session 导航 - 移到已完成
   - 关键指标 - 代码量、测试数、编译结果

**文档规范检查**：
- ✅ 文件名小写英文
- ✅ 标题不跳级（# → ## → ###）
- ✅ 代码块标注语言（\`\`\`rust）
- ✅ 超过 300 行需要拆分
- ✅ 更新时间戳

---

### Step 6: 生成提交信息

**参考文档**：
- [`docs/rules/commits.md`](../../../docs/rules/commits.md) - 提交信息格式规范

**提交信息格式**：

```
<type>: <简短描述>（50 字以内）

## 核心功能
- 实现功能 1
- 实现功能 2

## 文件变更
新增：X
修改：Y
删除：Z

## 质量指标
✓ cargo fmt ✓
✓ cargo clippy ✓
✓ cargo test ✓ (N/N 通过)
✓ cargo build ✓

进度：X% → Y%
```

**提交类型决定规则**：

| 改动内容 | 提交类型 | 例子 |
|---------|--------|------|
| 新增/修改 Rust 代码 | `feat` | 新功能实现 |
| 修改文档 | `docs` | Session 记录更新 |
| 新增测试 | `test` | 测试代码 |
| 依赖/配置 | `chore` | Cargo.toml 修改 |

**关键检查**：
- ✅ 简短描述 50 字以内
- ✅ 详细说明包含文件变更统计
- ✅ 质量指标显示四项检查结果
- ✅ 进度标记格式正确

---

**更新时间**：2026-06-09
