/// 对于某些用例，匹配枚举时match很尴尬。例如：
/// let optional = Some(7);
///
/// match optional {
///     Some(i) => {
///         println!("This is a really long string and `{:?}`", i);
///         // ^ Needed 2 indentations just so we could destructure
///         // `i` from the option.
///     },
///     _ => {},
///     // ^ 是必需的，因为 `match` 是穷尽的。
///     // 看起来不像浪费空间？
/// };
/// if let对于这个用例来说更干净，此外还允许指定各种故障选项：

fn main() {
    // All have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let` 结构如下： "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // 如果需要指定失败，请使用 else：
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // 解构失败。改成失败案例。
        println!("Didn't match a number. Let's go with a letter!");
    }

    // 提供改变的失败条件
    let i_like_letters = true;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
        // 解构失败。评估“else if”条件以查看是否采用备用失败分支：
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // 如果该条件评估为假。该分支是默认分支
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    // 以同样的方式，if let可以用来匹配任何枚举值：
    // Create example variables
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // 变量 c 与具有Foo::Qux的值  匹配
    // 与前面示例中的 Some() 类似
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // 绑定也适用于“if let”
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }
    // 另一个好处是 if let 允许我们匹配非参数化枚举变体。
    // 即使在枚举未实现或派生 PartialEq 的情况下也是如此。
    // 在这种情况下， if Foo::Bar == a 将无法编译，因为枚举的实例不能相等，但 if let 将继续工作。
    let a = Foo::Bar;
    let aa = Foo1::Bar;

    // 由于实现了PartialEq，所以可以比较
    if aa == Foo1::Bar {
        println!("a is foobar");
    }
    // if Foo::Bar == a this causes a compile-time error. Use `if let` instead.
    if let Foo::Bar = a {
        println!("a is foobar");
    }
}

// Our example enum
enum Foo {
    Bar,
    Baz,
    Qux(u32),
}
#[derive(PartialEq)]
enum Foo1 {
    Bar,
    Baz,
    Qux,
}
