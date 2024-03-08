/// Rust 有两种不同类型的常量，可以在任何范围（包括全局范围）中声明。两者都需要显式类型注释：
/// const：不可更改的值（常见情况）。
/// 常量变量在编译时计算，并且无论在何处使用它们的值都会内联，内存中没有固定地址
/// static：一个可能 mut 具有 'static 生命周期的变量。静态生命周期是推断出来的，不必指定。访问或修改可变静态变量是unsafe。
/// 它们在使用时不会内联，并且具有实际的关联内存位置。这对于不安全和嵌入式代码非常有用，并且变量在整个程序执行过程中都存在。
///
/// 当全局范围的值没有理由需要对象标识时，const通常是首选

// 全局变量是在所有其他作用域之外声明的。
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
}
