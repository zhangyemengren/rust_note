// NOTE: `&'b data.0` and `'x: {` is not valid syntax!
struct Closure<F> {
    data: (u8, u16),
    func: F,
}

// 取消注释 并注释下方hrtb实现 尝试运行
// impl<F> Closure<F>
//     // where F: Fn(&'??? (u8, u16)) -> &'??? u8, 不使用hrtb 无法表示生命周期
// {
//     fn call<'a>(&'a self) -> &'a u8 {
//         (self.func)(&self.data)
//     }
// }
// fn do_it<'b>(data: &'b (u8, u16)) -> &'b u8 { &'b data.0 }

impl<F> Closure<F>
where
    F: for<'x> Fn(&'x (u8, u16)) -> &'x u8,
{
    fn call(&self) -> &u8 {
        (self.func)(&self.data)
    }
}
// 以用hrbt表示任意生命周期
fn do_it(data: &(u8, u16)) -> &u8 {
    &data.0
}

fn main() {
    let clo = Closure {
        data: (0, 1),
        func: do_it,
    };
    println!("{}", clo.call());
}
