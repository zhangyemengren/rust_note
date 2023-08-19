/// 函数是使用fn关键字声明的。它的参数有类型注释，就像变量一样，
/// 并且，如果函数返回一个值，则必须在箭头后面指定返回类型->。
/// 函数中的最终表达式将当作返回值。或者，该return语句可用于在函数内部（甚至在循环或if语句内部）更早地返回值。


// 与C/C++不同，函数定义的顺序没有限制
fn main() {
    // 我们可以在这里使用这个函数，稍后在某个地方定义它
    fizzbuzz_to(100);
}

// 返回布尔值的函数
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // 提前返回的情况
    if rhs == 0 {
        return false;
    }

    // 这是一个表达式，这里不需要`return`关键字
    lhs % rhs == 0
}

// 不返回值的函数实际上返回单位类型`()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// 当函数返回 `()` 时，返回类型可以省略签名
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}