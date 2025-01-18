# Higher-Rank Trait Bounds (HRTBs) 高阶特征界限

## 参考资料
- [Rust RFC: Higher-ranked trait bounds](https://rust-lang.github.io/rfcs/0387-higher-ranked-trait-bounds.html)
- [Rustonomicon: Higher-Rank Trait Bounds](https://doc.rust-lang.org/nomicon/hrtb.html)

## 概述
HRTB（高阶特征界限）主要用于处理涉及闭包和生命周期的场景。它能够帮助我们更精确地控制和描述生命周期的关系，特别是在处理闭包参数时。

## 使用场景
HRTB多见于以下场景：
1. 处理闭包参数
2. 需要灵活处理生命周期的trait实现
3. 泛型函数中需要保证生命周期的一致性

## 示例代码
```rust
// 这段代码会报错
fn foo<'a, T: Fn(&'a i32) -> &'a i32>(func: T) {
    for i in 0..5 {
        println!("{}", func(&i));
    }
}

// 使用HRTB的正确方式
fn foo_hrtb<T: for<'a> Fn(&'a i32) -> &'a i32>(f: T) {
    for i in 0..5 {
        println!("{}", f(&i));
    }
}

fn main() {
    foo(|i| i);
    foo_hrtb(|i| i);
}
```

## 更多查看 examples
