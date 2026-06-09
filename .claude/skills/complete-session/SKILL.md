---
name: complete-session
description: 完整完成当前 session 的所有工作，从理解目标到提交代码
---

TRIGGER when:
- 完成当前 session
- 完成这个 session
- 完成 session 任务
- 完成当前 session 的任务

执行前检查（失败则停止，提示用户处理后重试）：

1. 检查是否在 Rust 项目中
   ```bash
   test -f Cargo.toml && cargo --version > /dev/null
   ```
   失败 → 提示：「当前不在有效的 Rust 项目目录中」

2. 检查工作区是否健康（无冲突、无错误）
   ```bash
   git diff --check > /dev/null 2>&1
   ```
   失败 → 提示：「工作区存在冲突或其他问题，请先处理」

步骤列表：

Step 1: 读取 Session 目标
Step 2: 实现代码功能
Step 3: 编写测试代码
Step 4: 验证代码正确性
Step 5: 更新相关文档
Step 6: 生成提交信息

执行命令：

```
/complete-session
```

## 工作流说明

这个 Skill 完整完成一个 session 的所有工作，包括：

1. 读取 Session 目标
2. 实现代码功能
3. 编写测试代码
4. 验证代码正确性
5. 更新相关文档
6. 生成提交信息

具体的操作步骤见 `.claude/commands/complete-session/COMMAND.md`

### 预期输出

✅ 成功完成会输出：
- 所有质量检查结果（fmt/clippy/test/build）
- 完整的、已格式化的提交信息
- 下一步操作提示（用户手动 git commit）

❌ 如遇失败会输出：
- 失败项和错误信息
- 修复建议
- 重新调用 Skill 的提示

## 注意事项

- 提交信息生成后由用户手动执行 `git commit`，不自动提交
- 如任何步骤失败，应根据错误信息修复后重新调用此 Skill
- 所有具体操作详见 `.claude/commands/complete-session/COMMAND.md`
