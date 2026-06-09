# 测试规范 | testing.md

> 单元测试的规则和最佳实践
> 最后更新：2026-06-09

---

## 核心原则

- ✅ 每个公开函数都应有对应的单元测试
- ✅ 测试应验证成功和失败的路径
- ✅ 测试应该独立且可重复执行
- ✅ 提交代码前必须 `cargo test` 全部通过

---

## 测试框架

### 位置
```
src/module/file.rs
    ↓
末尾添加 #[cfg(test)] 模块：

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_xxx() { ... }
}
```

### 执行
```bash
cargo test              # 运行所有测试
cargo test --lib       # 仅库测试
cargo test module::    # 运行特定模块的测试
cargo test -- --nocapture  # 显示 println! 输出
```

---

## 命名规范

### 规则

```
test_{被测试功能}_{场景描述}_{预期结果}
```

### 示例

**好的命名**：
- `test_parse_heading_success` - 解析标题成功
- `test_render_preview_preserves_italic` - 预览保留斜体
- `test_buffer_set_content_updates_length` - 设置内容更新长度
- `test_parse_empty_string_returns_empty` - 解析空字符串返回空

**不好的命名**：
- `test_parse` - 太模糊
- `test_1` - 无含义
- `should_work` - 不符合 Rust 约定
- `testParse` - 混合大小写

---

## 测试编写清单

### ✅ 必做

#### 1. 成功路径测试

最基础的场景：正常输入 → 正确输出

```rust
#[test]
fn test_parse_heading() {
    let markdown = "# Hello";
    let html = Parser::to_html(markdown);
    assert!(html.contains("<h1>"));
    assert!(html.contains("Hello"));
}
```

#### 2. 边界/失败路径测试

异常情况的处理

```rust
#[test]
fn test_render_empty() {
    let html = Renderer::render("");
    assert_eq!(html, "");
}

#[test]
fn test_buffer_invalid_position() {
    // 验证错误处理
    let buffer = Buffer::new();
    // 对无效位置的操作应返回错误
}
```

#### 3. 测试注释说明意图

```rust
#[test]
fn test_render_preview_preserves_italic() {
    // 纯文本预览模式应保留 *italic* 格式，
    // 仅去掉行首的标题标记（#），
    // 这样用户可以看到原始的 Markdown 内容
    let markdown = "*italic text*";
    let preview = Renderer::render_preview(markdown);
    assert_eq!(preview, "*italic text*");
}
```

### ❌ 禁止

1. **无意义的测试**
   ```rust
   #[test]
   fn test_new() {
       let buffer = Buffer::new();
       // 这个测试什么都没验证！
   }
   ```

2. **多个 assert 测试不相关的功能**
   ```rust
   #[test]
   fn test_complex() {
       // 不要！这是两个测试
       assert!(parse_heading("# Hello").contains("<h1>"));
       assert_eq!(render("").len(), 0);
   }
   ```

3. **依赖外部状态**
   ```rust
   #[test]
   fn test_with_file() {
       // 不要！依赖文件系统，测试不可重复
       let content = std::fs::read_to_string("test.md").unwrap();
       ...
   }
   ```

4. **在测试中使用 panic/unwrap**
   ```rust
   #[test]
   fn test_parse() {
       let result = Parser::parse("# Hello").unwrap();  // 不要！
       assert!(result.html.contains("<h1>"));
   }
   
   // 改为：
   #[test]
   fn test_parse() {
       let result = Parser::parse("# Hello");
       assert!(result.is_ok());
       assert!(result.unwrap().html.contains("<h1>"));
   }
   ```

---

## 测试覆盖目标

### 按模块的覆盖率要求

| 模块 | 最低覆盖率 | 优先级 | 说明 |
|------|----------|------|------|
| editor/buffer | ≥ 80% | 🔴 高 | 核心数据结构 |
| markdown/parser | ≥ 90% | 🔴 高 | 外部库集成 |
| markdown/renderer | ≥ 90% | 🔴 高 | 核心渲染逻辑 |
| ui/ | ≥ 50% | 🟡 中 | UI 框架 |
| 其他 | ≥ 60% | 🟡 中 | 一般模块 |

### 如何度量

```bash
# 使用 tarpaulin（需要安装）
cargo tarpaulin --out Html

# 或使用 llvm-cov（更准确）
cargo llvm-cov --html
```

---

## 常见测试模式

### 1. 验证输出内容

```rust
#[test]
fn test_render_contains_tags() {
    let html = Renderer::render("# Hello");
    assert!(html.contains("<h1>"));
}
```

### 2. 验证等值

```rust
#[test]
fn test_render_preview_exact() {
    let preview = Renderer::render_preview("# Hello");
    assert_eq!(preview, "Hello");
}
```

### 3. 验证错误

```rust
#[test]
fn test_parse_error() {
    let result = Parser::parse("invalid");
    assert!(result.is_err());
}
```

### 4. 验证集合

```rust
#[test]
fn test_buffer_multiple_lines() {
    let mut buffer = Buffer::new();
    buffer.set_content("line1\nline2\nline3");
    assert_eq!(buffer.line_count(), 3);
}
```

---

## 提交前检查清单

必须通过所有以下检查：

```bash
cargo fmt          # 代码格式化
cargo clippy       # 静态检查
cargo test         # 单元测试
cargo build        # 编译
```

---

## 当前项目的测试状态

```
editor/buffer          3 个测试 ✓
markdown/parser        3 个测试 ✓
markdown/renderer      5 个测试 ✓
────────────────────────────────
总计：11 个测试，全部通过 ✓
```

---

## 常见问题

### Q: 测试应该多详细？
**A**: 恰到好处。验证关键行为，不需要测试编译器已验证的部分。

### Q: 私有函数需要测试吗？
**A**: 通常通过公开函数的测试间接验证。如果私有函数很复杂，可以直接测试。

### Q: 如何测试异步代码？
**A**: 目前项目还没有异步代码。阶段二会引入 tokio，那时使用 `#[tokio::test]`。

### Q: 集成测试放在哪？
**A**: `tests/` 目录（阶段四实现）。单元测试放在 `#[cfg(test)]` 模块。

### Q: 如何跳过某个测试？
**A**: 使用 `#[ignore]` 属性，然后 `cargo test -- --ignored` 来运行。

---

## 参考资源

- [Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust By Example - Testing](https://doc.rust-lang.org/rust-by-example/testing.html)
- [本项目代码规范](./code.md)

---

**最后更新**：2026-06-09 | **下一阶段**：集成测试（阶段四）
