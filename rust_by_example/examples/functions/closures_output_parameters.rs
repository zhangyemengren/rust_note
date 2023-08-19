/// 闭包作为输入参数是可能的，因此返回闭包作为输出参数也应该是可能的。
/// 然而，根据定义，匿名闭包类型是未知的，因此我们必须使用 impl Trait 来返回它们。
/// 返回闭包的有效特征是：
/// Fn
/// FnMut
/// FnOnce
/// 除此之外，必须使用 move 关键字，它表示所有捕获均按值发生。
/// 这是必需的，因为一旦函数退出，任何通过引用的捕获都会被删除，从而在闭包中留下无效的引用。

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();
    // 闭包中返回闭包
    let fn_closure = (|| {
        let text = "closure".to_owned();

        move || println!("This is a: {}", text)
    })();

    fn_plain();
    fn_mut();
    fn_once();
    fn_closure();
}
