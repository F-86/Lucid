# Rust 学习路线 | rust.md

> 对应项目四阶段的 Rust 核心概念学习
> 最后更新：2026-06-09

---

## 阶段一：所有权 & 模式匹配

**学习重点**：
- 所有权转移（Message 枚举）
- 借用（view(&self) vs update(&mut self)）
- 模式匹配（enum 分支）

**在项目中体现**：
- Message 枚举定义：所有权转移的实例
- view() 函数：只借用引用
- update() 函数：可变借用修改状态

---

## 阶段二：生命周期 & 迭代器 & 异步

**学习重点**：
- 生命周期参数（RopeSlice<'_>）
- 迭代器（pulldown-cmark 事件流）
- 异步编程（Task::perform）

**在项目中体现**：
- 文本缓冲区返回切片（生命周期）
- Markdown 解析器流式处理（迭代器）
- 文件 IO 异步操作（async/await）

---

## 阶段三：Trait & 泛型 & 高级借用

**学习重点**：
- Trait 定义与实现
- 泛型约束（Trait bounds）
- 复杂借用（Arc<RwLock<T>>）

**在项目中体现**：
- 自定义 widget 实现 Trait
- 泛型参数在 Message 中
- 共享可变状态的并发安全

---

## 阶段四：工程化 & 测试 & 文档

**学习重点**：
- 错误处理（thiserror）
- 测试（单元 + 集成）
- 文档（rustdoc）

**在项目中体现**：
- 自定义错误类型
- 为 editor/ 模块写单元测试
- 函数 rustdoc 注释

---

## 常用 Rust 概念速查

| 概念 | 用途 | 在项目中 |
|---|---|---|
| 所有权 | 内存管理 | Message 枚举 |
| 生命周期 | 引用有效期 | RopeSlice<'_> |
| Trait | 多态抽象 | Custom widget |
| 异步 | 非阻塞 IO | Task::perform |
| 错误处理 | 可恢复错误 | Result<T, E> |

---

**更多详情**：查看具体阶段的详细规划（`docs/intro/plan.md`）
