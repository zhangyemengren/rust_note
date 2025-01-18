// @ edition: 2021 可以编译 但 edition: 2024 可能会报错
// fn foo<'t>(_: &'t ()) -> impl Sized {}

use std::fmt::Debug;

// 解决方案 使用精确捕获 Captures nothing.
fn foo<'t>(_: &'t ()) -> impl use<> + Sized {}

fn bar(x: ()) -> impl Sized + 'static {
    foo(&x)
}

#[allow(dead_code)]
struct Ty<'a, 'b>(&'a (), &'b ());

impl<'a, 'b> Ty<'a, 'b> {
    // 可以精确捕获 只捕获了'a
    fn foo2(x: &'a (), _: &'b ()) -> impl use<'a> + Sized + Debug {
        x
    }
}

fn main() {
    bar(());
    let x = Ty::<'_, '_>::foo2(&(), &());
    println!("{:?}", x);
}
