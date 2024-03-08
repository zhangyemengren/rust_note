/// 该for in构造可用于迭代Iterator.
/// 创建迭代器最简单的方法之一是使用范围表示法a..b。这会产生从a（包含）到b（不包含）的值，步长为 1。

fn main() {
    // 让我们使用for而不是while编写 FizzBuzz。
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    // 或者，a..=b可用于两端都包含在内的范围。下式式可以写成：1..=100
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
    // for in 构造能够通过多种方式与迭代器交互。
    // 正如有关 Iterator 特征的部分中所讨论的，默认情况下 for 循环会将 into_iter 函数应用于集合。
    // 然而，这并不是将集合转换为迭代器的唯一方法。
    // into_iter、iter 和 iter_mut 都通过提供内部数据的不同视图，以不同的方式处理集合到迭代器的转换。

    // iter - 通过每次迭代借用集合中的每个元素。从而使集合保持不变并可在循环后重用。
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    // into_iter - 这会消耗集合，以便在每次迭代时提供准确的数据。一旦集合被消耗，它就不再可供重用，因为它已在循环内“移动”。
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // println!("names: {:?}", names); // names不在可用因为值已经移动

    // iter_mut- 这可变地借用集合的每个元素，允许就地修改集合。
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);

    // 在上面的代码片段中，请注意match分支的类型，这是迭代类型的关键区别。类型的差异当然意味着能够执行的不同动作。
}
