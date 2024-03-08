#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    // 可以添加一个守卫来过滤匹配
    let temperature = Temperature::Celsius(15);
    // ^ TODO try different values for `temperature`

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        // `if condition` 是一个守卫
        Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is below 86 Fahrenheit", t),
    }

    let number: u8 = 11;
    // 请注意，编译器在检查匹配表达式是否涵盖了所有模式时，不会检查带有守卫的匹配。
    // 也就是编译器不会检查前两条是否已经涵盖所有匹配，所以这里需要加上 _ => unreachable!()，虽然事实上不会执行到这里。
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => unreachable!("Should never happen."),
        // TODO ^ uncomment to fix compilation
    }
}
