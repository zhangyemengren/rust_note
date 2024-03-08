/// Rust 通过match关键字提供模式匹配，可以像 C 一样使用switch。
/// 至少需要列出一个匹配并且必须覆盖所有可能的值。

fn main() {
    let number = 151;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // 匹配单个值
        1 => println!("One!"),
        // 匹配多个值
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 匹配包含范围
        13..=19 => println!("A teen"),
        // 处理其余cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }

    let boolean = true;
    // match也是一种表达式
    let binary = match boolean {
        // match 必须覆盖所有可能出现的情况
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}
