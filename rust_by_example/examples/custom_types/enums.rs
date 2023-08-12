#![allow(dead_code)]
/// enum 关键字允许创建一个类型，该类型可能是几种不同变体之一。
/// 任何作为struct有效的变体在枚举中也有效。


// 创建一个 `enum` 对网络事件进行分类。
// 请注意名称和类型信息如何一起指定变体：
// “PageLoad != PageUnload” 且 “KeyPress(char) != Paste(String)”。
// 每个都是不同且独立的。
enum WebEvent {
    // An `enum` 变体类似单元结构`,
    PageLoad,
    PageUnload,
    // 类似元组结构
    KeyPress(char),
    Paste(String),
    // 或 c-like 结构体
    Click { x: i64, y: i64 },
}
/// 如果使用类型别名，则可以通过其别名引用每个枚举变体。
/// 如果枚举的名称太长或太通用，并且您想要重命名它，这可能很有用。
#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}
// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
// Self别名
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}
/// C-like
// 隐式识别的吗，枚举 (starts at 0)
#[derive(Debug)]
enum Number {
    Zero,
    One,
    Two,
}

// 显式识别的枚举
enum Color {
    Red = 0xff0000,
    Green, // 不指定，增加1
    Blue = 0x0000ff,
}
fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    {
        // 可以使用该use声明将枚举的变体引入作用域。
        // 这样就可以直接使用变体的名称，而不使用路径符号。
        use crate::WebEvent::*;
        let unload2 = PageUnload;
        inspect(unload2);
    }

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let x = Operations::Add;
    println!("{:?}", x);

    // `enums` 可以转换为整数。
    println!("zero is {} {:?}", Number::Zero as i32, Number::Zero);
    println!("one is {}", Number::One as u32);
    // 0填充到6位
    println!("red are #{:06x}", Color::Red as i32);
    println!("green are #{:06x}", Color::Green as i32);
    println!("blue are #{:06x}", Color::Blue as i32);
}
