# 代码规范 | code.md

> 写代码时的规则检查清单
> 最后更新：2026-06-09

---

## ✅ 必做

### 基础
- [ ] 代码用英文，注释用中文
- [ ] 函数签名有 rustdoc（`///`）
- [ ] 显式生命周期参数（pub fn foo<'a>(&'a str)）
- [ ] 返回 Result，不 unwrap
- [ ] 编译前 `cargo fmt && cargo clippy`

### 函数
```rust
/// 这个函数做什么
/// 
/// # 参数
/// - `x`：什么的什么
///
/// # 返回值
/// Ok(result) 或 Err(错误)
pub fn do_something(x: &str) -> Result<String, MyError> {
    // 实现...
}
```

### 模块
- [ ] editor/ 纯 Rust（不导入 iced）
- [ ] 模块内清晰的职责分工
- [ ] 无循环依赖

### 测试
- [ ] 写单元测试（`#[cfg(test)]`）
- [ ] 至少测试成功和失败路径
- [ ] 测试注释说明意图

---

## ❌ 禁止

- ❌ panic/unwrap 在业务逻辑
- ❌ 编译后有 clippy 警告
- ❌ editor/ 导入 iced 相关
- ❌ public 函数没有生命周期标注

---

## 提交前检查

```bash
cargo fmt && cargo clippy && cargo test
```
