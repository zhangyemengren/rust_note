/// 发散的函数永远不会返回。它们使用 进行标记!，这是一个空类型。
/// fn foo() -> ! {
///     panic!("This call never returns.");
/// }
/// 与所有其他类型相反，该类型无法实例化，因为该类型可以具有的所有可能值的集合是空的。
/// 请注意，它与 () 类型不同， () 类型只有一个可能的值。
/// 例如，该函数照常返回，尽管返回值中没有任何信息。
fn some_fn() {
    ()
}

fn main() {
    let _a: () = some_fn();
    println!("This function returns and you can see this line.");

    // 与^此函数相反，它永远不会将控制权返回给调用者。
    // !类型是实验性的因此需要nightly版本才能启用
    // #![feature(never_type)]
    // let x: ! = panic!("This call never returns.");
    // println!("You will never see this line!");

    // 尽管这看起来像是一个抽象的概念，但实际上它非常有用并且通常很方便。
    // 这种类型的主要优点是它可以转换为任何其他类型，因此可以在需要精确类型的地方使用，
    // 例如在匹配分支中。这允许我们编写如下代码：
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // 请注意，由于“addition”变量的类型，此匹配表达式的返回类型必须为 u32。
            let addition: u32 = match i%2 == 1 {
                // “i”变量的类型为 u32，这完全没问题。
                true => i,
                // 另一方面，“继续”表达式不返回 u32，
                // 但它仍然没问题，因为它永远不会返回，因此不会违反匹配表达式的类型要求。
                false => continue,
            };
            acc += addition;
        }
        acc
    }

    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}