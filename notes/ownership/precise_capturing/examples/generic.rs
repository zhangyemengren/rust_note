//
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
