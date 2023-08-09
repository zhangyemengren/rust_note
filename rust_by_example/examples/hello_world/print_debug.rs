/// 所有想要使用std::fmt格式的traits类型都需要一个可打印的实现。仅针对库中的类型提供自动实现std。所有其他的都必须以某种方式手动实现。
/// fmt::Debug trait使得这非常简单。所有类型都可以 derive（自动创建）fmt::Debug实现。fmt::Display则必须手动实现。

// 派生“Structure”的“fmt::Debug”实现。
// 是一个包含单个“i32”的结构。
#[derive(Debug)]
struct Structure(i32);

// 在结构“Deep”中放置一个“Structure”。使其也可打印
#[derive(Debug)]
struct Deep(Structure);

#[warn(dead_code)]
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));
    // 所有std库类型也可以自动打印{:?}：
    // 所以fmt::Debug绝对可以打印，但牺牲了一些优雅。Rust 还提供了“漂亮的打印” {:#?}。

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}