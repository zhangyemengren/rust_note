# 精确捕获

## 参考资料
- [Rust RFC 3617: Precise Capture](https://rust-lang.github.io/rfcs/3617-precise-capturing.html)

## 概述
此 RFC 添加了use<..>用于指定应在类似 RPIT（Return Position impl Trait）的不透明类型中捕获哪些通用参数的语法impl Trait，例如impl use<'t, T> Trait。这解决了过度捕获的问题，并将允许生命周期捕获规则 2024 在 Rust 2024 中为 RPIT 完全稳定。

## 使用场景
- 防止生命周期过度捕获
- 捕获通用参数