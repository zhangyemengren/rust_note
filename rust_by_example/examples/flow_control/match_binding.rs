/// 间接访问变量会导致无法在不重新绑定的情况下分支和使用该变量。
/// match 提供了将值与名称绑定的 @ 符号：
// A function `age` which returns a `u32`.
fn age() -> u32 {
    25
}

fn some_number() -> Option<u32> {
    Some(51)
}
fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // 可以直接 `match` 1 ...= 12，但这样的话，孩子的年龄是多少？
        // 孩子会是什么年龄？取而代之的是，绑定到 `n` 以获得
        // 序列 1 ...=12。现在就可以报告年龄了。
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // 此处也可以使用守卫
        n if n > 20 && n < 30 => println!("I'm an adult of age {:?}", n),
        // Nothing bound. Return the result.
        n => println!("I'm an old person of age {:?}", n),
    }

    // 您还可以使用绑定来 "重组 "枚举变量，例如 Option：
    match some_number() {
        // 得到 `Some` 变量，如果它的值与 `n` 绑定，则匹配、
        // 等于 42。
        Some(n @ 42) => println!("The Answer: {}!", n),
        // 此处也可以使用守卫
        Some(n) if n == 43 => println!("The Answer from guard: {}!", n),
        // 守卫与绑定结合
        Some(n @ 44..=50) if n != 49 => println!("The Answer from guard and binding: {}!", n),
        // 绑定能做但守卫不行的例子 @绑定多个变量
        z @ Some(y @ 51) => println!("The Answer from only binding: {:?} {:?}!", z, y),
        // Match any other number.
        Some(n) => println!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        _ => (),
    }
}
