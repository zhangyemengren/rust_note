/// 高阶函数
/// Rust 提供高阶函数 (HOF)。这些函数采用一个或多个函数 且/或 产生更有用的函数。
/// HOF 和惰性迭代器赋予 Rust 函数式风格。

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // 命令式方法
    // 声明累加器变量
    let mut acc = 0;
    // Iterate: 0, 1, 2, ... to infinity
    for n in 0.. {
        // 平方
        let n_squared = n * n;

        if n_squared >= upper {
            // 超过上限则中断循环
            break;
        } else if is_odd(n_squared) {
            // 如果是奇数则累加值
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // 函数式方法
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // 所有自然数的平方
        .take_while(|&n_squared| n_squared < upper) // 低于上限
        .filter(|&n_squared| is_odd(n_squared)) // 是奇数
        .sum(); // Sum them
    println!("functional style: {}", sum_of_squared_odd_numbers);
}
