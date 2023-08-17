/// Rust 程序（大部分）由一系列语句组成：

fn main() {
    // Rust 中有几种语句。 最常见的两种是声明变量绑定和 使用 ; 与表达式一起使用：

    // variable binding
    let x = 5;

    // expression;
    x;
    x + 1;
    15;
    // 块也是表达式，因此它们可以用作赋值中的值。
    // 块中的最后一个表达式将被分配给位置表达式，例如局部变量。但是，如果块的最后一个表达式以分号结尾，则返回值为()。

    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 该表达式将被分配给“y”
        x_cube + x_squared + x
    };

    let z = {
        // 分号抑制了该表达式，并将“()”分配给“z”
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}