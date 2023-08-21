/// 泛型是将类型和功能概括到更广泛的情况的主题。这对于以多种方式减少代码重复非常有用，但可能需要相当复杂的语法。
/// 也就是说，泛型需要非常小心地指定泛型类型实际上被认为是有效的类型。泛型最简单且最常见的用途是用于类型参数。
/// 通过使用尖括号和大驼峰式大小写将类型参数指定为泛型：<Aaa, Bbb, ...>。
/// “通用类型参数”通常表示为 <T>。在 Rust 中，“泛型”还描述了任何接受一个或多个泛型类型参数 <T> 的事物。
/// 指定为泛型类型参数的任何类型都是泛型，其他所有类型都是具体的（非泛型）。
/// 例如，定义一个名为 foo 的泛型函数，它接受任何类型的参数 T：
/// fn foo<T>(arg: T) { ... }
/// 由于 T 已使用 <T> 指定为泛型类型参数，因此在此处使用 (arg: T) 时，它被视为泛型。即使 T 先前已被定义为结构体，情况也是如此。
/// 此示例显示了一些实际语法：

// 具体类型“A”。
struct A;

// 在定义类型“Single”时，第一次使用“A”之前不加“<A>”。
// 因此，“Single”是一个具体类型，“A”的定义如上。
struct Single(A);
//            ^ 这是“Single”第一次使用“A”类型。

// 这里，“<T>”先于第一次使用“T”，因此“SingleGen”是泛型类型。
// 因为类型参数“T”是通用的，所以它可以是任何东西，包括顶部定义的具体类型“A”。
struct SingleGen<T>(T);

fn main() {
    // “Single”是具体的，明确采用“A”。
    let _s = Single(A);

    // 创建类型为 `SingleGen<char>` 的变量 `_char`
    //  并为其赋予值 `SingleGen('a')` 这里，`SingleGen` 有一个显式指定的类型参数。
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen` 还可以隐式指定类型参数：
    let _t    = SingleGen(A); // Uses `A` defined at the top.
    let _i32  = SingleGen(6); // Uses `i32`.
    let _char = SingleGen('a'); // Uses `char`.
}