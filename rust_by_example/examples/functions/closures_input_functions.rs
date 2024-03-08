/// 由于闭包可以用作参数，因此您可能想知道函数是否也可以这样。他们确实可以！
/// 如果您声明一个采用闭包作为参数的函数，则任何满足该闭包特征边界的函数都可以作为参数传递。

// 定义一个函数，该函数采用以“Fn”为界的通用“F”参数，并调用它
fn call_me<F: Fn()>(f: F) {
    f();
}

// 定义满足“Fn”边界的包装函数
fn function() {
    println!("I'm a function!");
}

fn main() {
    // 定义满足“Fn”边界的闭包
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}
