use std::fmt;
/// fmt::Debug看起来很难紧凑和干净，因此定制输出外观通常是有利的。
/// 这是通过手动实现 来完成的 fmt::Display，它使用{}打印标记。实现它看起来像这样：
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

struct List(Vec<i32>);
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用元组的下标来获取值，并创建一个 `vec` 的引用。
        let vec = &self.0;
        write!(f, "[")?;
        // 使用 `v` 对 `vec` 进行迭代，并用 `count` 记录迭代次数。
        for (count, v) in vec.iter().enumerate() {
            // 对每个元素（第一个元素除外）加上逗号。
            // 使用 `?` 或 `try!` 来返回错误。
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        // 加上配对中括号，并返回一个 fmt::Result 值。
        write!(f, "]")
    }
}
fn main() {
    // 因为没有适合所有类型的理想样式，并且std lib也不会规定一种样式。
    // fmt::Display没有为任何其他通用容器实现。然后 fmt::Debug必须用于这些一般情况。
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    // Error. `Debug` 和 `Display`都已经 implemented, 但 `{:b}`
    // 需要 impl `fmt::Binary`
    // println!("What does Point2D look like in binary: {:b}?", point);
    // 因为 fmt::Display已实施 但 fmt::Binary尚未实施，因此无法使用。
    // std::fmt有很多这样的traits，每个都需要自己的实现。
}
