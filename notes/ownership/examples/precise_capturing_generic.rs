///# 精确捕获
///
/// ## 参考资料
/// - [Rust RFC 3617: Precise Capture](https://rust-lang.github.io/rfcs/3617-precise-capturing.html)
///
/// ## 概述
/// 此 RFC 添加了use<..>用于指定应在类似 RPIT（Return Position impl Trait）的不透明类型中捕获哪些通用参数的语法impl Trait，例如impl use<'t, T> Trait。这解决了过度捕获的问题，并将允许生命周期捕获规则 2024 在 Rust 2024 中为 RPIT 完全稳定。
///
/// ## 使用场景
/// - 防止生命周期过度捕获
/// - 捕获通用参数
/// 

// fn foo<'t, T, U>(_: &'t (), _: T, y: U) -> impl use<U> + Sized { y }

// 除了类型和生命周期参数之外，我们还可以使用它来捕获通用 const 参数。
fn foo2<'t, const C: u8>(_: &'t ()) -> impl use<C> + Sized {
    C
}
// 结合hrtb
fn foo3<T>(_: T) -> impl for<'a> FnOnce(&'a ()) + use<T> {
    |&()| ()
}

struct S<T, const C: usize>((T, [(); C]));
impl<T, const C: usize> S<T, C> {
    // These generic parameters are in scope.
    fn f_implicit<U>() -> impl Sized {}

    // 捕获范型
    fn f_explicit<U>() -> impl Sized + use<T, U, C> {}
}

fn main() {
    // foo(&(), 0, 1);
    foo2::<0>(&());
    foo3(0)(&());
    S::<i32, 1>::f_implicit::<u32>();
    S::<i32, 1>::f_explicit::<u32>();
}
