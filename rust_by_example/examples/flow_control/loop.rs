#![allow(unreachable_code)]
/// Rust 提供了一个loop关键字来指示无限循环。
/// 该break语句可用于随时退出循环，而该 continue语句可用于跳过迭代的其余部分并开始新的迭代。

fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // 无限循环
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 跳过本次迭代的其余部分
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // 退出循环
            break;
        }
    }
    // 处理嵌套循环时可以中断或继续外部循环。
    // 在这些情况下，循环必须用一些“标签('label)”进行注释，并且该标签必须传递给break/continue 语句。
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // 这只会break内部循环
            //break;

            // 这break外循环
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    // Returning from loops
    // 循环的用途之一是重试操作直到成功。
    // 如果操作返回一个值，您可能需要将其传递给代码的其余部分：将其放在中断之后，它将由循环表达式返回。
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
