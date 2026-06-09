# 常用命令 | cmd.md

> 项目中常用的 Rust 命令和开发工作流
> 最后更新：2026-06-09

---

## 🔄 开发工作流

**每次完成代码后执行此流程**：

```bash
# 1. 编译检查
cargo check

# 2. 运行测试
cargo test

# 3. 格式化和静态检查
cargo fmt && cargo clippy

# 4. 查看更改
git status

# 5. 添加到暂存区
git add -A

# 完成！等待提交指令
```

---

## 开发命令

```bash
cargo run              # 启动应用
cargo test --lib      # 单元测试
cargo test --test '*' # 集成测试
```

## 检查命令

```bash
cargo fmt             # 格式化代码
cargo clippy          # 静态检查
cargo doc --open      # 生成并打开文档
```

## 构建命令

```bash
cargo build --release # 发布版本
cargo bundle          # 打包应用
```

## 诊断命令

```bash
RUST_BACKTRACE=1 cargo run  # 完整堆栈跟踪
cargo tree                  # 依赖树
cargo outdated              # 过期依赖
```

---

## ✅ 提交前检查清单

执行此命令确认所有检查通过：

```bash
cargo fmt && cargo clippy && cargo test
```

输出应该显示：
- ✓ 代码格式化完成
- ✓ clippy 检查通过
- ✓ test result: ok. N passed

---

## 📝 文档更新规则

代码完成后，根据更改类型更新对应文档：

| 完成的工作 | 需要更新 |
|---------|--------|
| 新功能实现 | `docs/track/status.md` + `docs/track/implementation.md` |
| 功能完成 | `docs/intro/plan.md` (标记完成) |
| Bug 修复 | `docs/track/implementation.md` (记录解决方案) |
| 架构优化 | `docs/intro/arch.md` + `docs/track/implementation.md` |

---

## 🎯 提交流程

```bash
# 1. 确认所有工作完成
git status  # 查看待提交文件

# 2. 告诉我"提交"

# 3. 我会给出具体的 commit 命令
git commit -m "描述你做了什么

详细说明

Co-Authored-By: Claude Haiku 4.5 <noreply@anthropic.com>"

# 4. 你执行该命令
```

---

**提示**：每次编码前查看本文件，快速了解工作流程
