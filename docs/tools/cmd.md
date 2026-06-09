# 常用命令 | cmd.md

> 项目中常用的 Rust 命令
> 最后更新：2026-06-09

---

## 开发

```bash
cargo run              # 启动应用
cargo test --lib      # 单元测试
cargo test --test '*' # 集成测试
```

## 检查

```bash
cargo fmt             # 格式化代码
cargo clippy          # 静态检查
cargo doc --open      # 生成并打开文档
```

## 构建

```bash
cargo build --release # 发布版本
cargo bundle          # 打包应用
```

## 诊断

```bash
RUST_BACKTRACE=1 cargo run  # 完整堆栈跟踪
cargo tree                  # 依赖树
cargo outdated              # 过期依赖
```

---

## 提交前检查清单

```bash
cargo fmt && cargo clippy && cargo test
```
