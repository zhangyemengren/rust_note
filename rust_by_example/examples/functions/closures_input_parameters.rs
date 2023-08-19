/// 作为输入参数
/// 虽然 Rust 可以选择如何在没有类型注解的情况下动态捕获变量，但在编写函数时不允许这种歧义。
/// 当将闭包作为输入参数时，必须使用几个traits之一来注解闭包的完整类型，并且它们由闭包对捕获的值执行的操作确定。
/// 按照限制递减的顺序，它们是：
/// Fn：闭包通过引用(&T)使用捕获的值
/// FnMut：闭包通过可变引用(&mut T)使用捕获的值
/// FnOnceT：闭包通过值(T) 使用捕获的值(T)
/// 在逐个变量的基础上，编译器将以尽可能限制最少的方式捕获变量。
/// 例如，考虑一个注解为 FnOnce的参数。这指定闭包可以通过&T、&mut T或T 进行捕获，但编译器最终将根据捕获的变量在闭包中的使用方式进行选择。
/// 这是因为如果可以进行转移，那么任何类型的借用也应该是可能的。请注意，反之则不然。如果参数注释为Fn，则不允许使用&mut T或 T捕获变量。不过，&T是允许的。
///


// Fn在以下示例中，尝试交换、FnMut、 和 的用法，FnOnce看看会发生什么：
// 将闭包作为参数并调用它的函数。
// <F> 表示 F 是“泛型参数”
fn apply<F>(f: F) where
// 该闭包不接受任何输入，也不返回任何内容。
    F: FnOnce() {
    // ^ TODO: Try changing this to `Fn` or `FnMut`.

    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
// The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` 从借用的数据创建自有数据
    let mut farewell = "goodbye".to_owned();

    // 捕获 2 个变量：引用“greeting”和 值“farewell”。
    let diary = || {
        // `greeting` 通过引用捕获：需要 `Fn`。
        println!("I said {}.", greeting);

        // `farewell` 通过可变引用捕获：现在需要`FnMut`。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // drop 会强制`farewell`按值捕获。现在需要“FnOnce”。
        mem::drop(farewell);
    };

    // 调用传入闭包的函数。
    apply(diary);

    // `double` 满足 `apply_to_3` 的特征界限
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}