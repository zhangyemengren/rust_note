/// 使用if-进行分支else与其他语言类似。
/// 与许多语言不同的是，布尔条件不需要用括号括起来，并且每个条件后面都跟有一个块。
/// if-else条件句是表达式，并且所有分支必须返回相同的类型。

fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    } // 普通表达式 没有分号

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        // 该表达式返回一个“i32”。
        10 * n
    } else {
        println!(", and is a big number, halve the number");

        // 该表达式也必须返回“i32”。
        n / 2
    }; //  不要忘记在这里加分号！所有“let”绑定都需要它。

    println!("{} -> {}", n, big_n);
}
