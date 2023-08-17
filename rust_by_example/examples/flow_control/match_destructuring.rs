

fn main() {
    // 解构
    let triple = (0, -2, 3);
    // TODO ^ Try different values for `triple`

    println!("Tell me about {:?}", triple);
    // Match 可用于解构元组
    match triple {
        // 解构第二个和第三个元素
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        (.., 2)  => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
        // `..` 可用于忽略元组的其余部分
        _      => println!("It doesn't matter what they are"),
        // `_` 意味着不将值绑定到变量
    }

    // 与元组一样，数组和切片可以这样解构：
    let array = [13, -2, 6];

    match array {
        // 将第二个和第三个元素绑定到各自的变量
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // 可以用 _ 忽略单个值
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        // 您也可以绑定一些并忽略其余的
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // 下面的代码无法编译
        // [-1, second] => ...

        // 或者将它们存储在另一个数组/切片中
        // （类型取决所匹配的值的值）
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // 组合这些模式，例如，我们可以绑定第一个和
        // 最后的值，将其余的值存储在一个数据库中
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }

    // enum的解构类似：
    let color = Color::RGB(122, 17, 40);
    // TODO ^ Try different variants for `color`

    println!("What color is it?");

    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                     c, m, y, k),
        // 不在需要额外的匹配，因为所有变体都已经过检查
    }

    // 对于指针，需要区分解构和取消引用，因为它们是不同的概念，其使用方式与 C/C++ 等语言不同。
    // 解除引用用途 *
    // 解构使用 &、ref、 和ref mut


    // 在 Rust 中，引用是一个轻量级的指针，它指向已经存在的数据。引用本身不会分配内存，它只是提供了对已存在数据的访问。
    // 因此，let reference = &4; 将创建一个指向整数字面量 4 的引用，并不会涉及到额外的内存分配。
    let reference = &4;

    match reference {
        // 使用模式匹配来解构这个引用并获取其内部的值。
        // 在这个例子中，你可以使用 &val 这样的模式，其中 val 将会被绑定到引用指向的值。
        // 这是因为 Rust 的匹配语法会根据模式的形状来解构数据，包括引用的解构。
        &references_val => println!("Got a value via destructuring: {:?}", references_val),
    }
    // match 语句默认是会移动值的 所以当reference是移动语义 且不想移动时可以匹配引用 这样解构时需要多一个&符号
    match &reference {
        &&data => println!("Got a value via destructuring: {:?}", data),
    }

    // 为避免使用 `&`，应在匹配前取消引用。
    match *reference {
        real_val => println!("Got a value via dereferencing: {:?}", real_val),
    }


    // 如果不是从引用开始呢？引用`是一个`&`。因为右边已经是引用了。
    // 这不是引用，因为右边不是引用。
    let _not_a_reference = 3;

    // Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    // Rust 正是为此提供了 `ref`。它修改了赋值，从而为元素创建一个引用
    // 这个值被赋与引用
    let ref _is_a_reference = 3;

    // 因此，通过定义 2 个不带引用的值，
    // 引用可以通过 `ref` 和 `ref mut` 获取。
    let value = 5;
    let mut mut_value = 6;

    // 使用 `ref` 关键字创建引用。
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // 类似地使用 `ref mut`。
    match mut_value {
        ref mut m => {
            // 得到一个引用。
            // 在添加任何内容前必须先取消引用
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }

    // 类似地，struct可以被解构，如下所示：
    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (11, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // 你可以重组结构体并重新命名变量
        // 顺序并不重要
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // 也可以忽略某些变量：
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // 这将导致错误：模式未提及字段 `x`.
        //Foo { y } => println!("y = {}", y),
    }
}

struct Foo {
    x: (u32, u32),
    y: u32,
}
#[allow(dead_code)]
enum Color {
    // 这 3 个仅通过其名称来指定
    Red,
    Blue,
    Green,
    // 这些将“u32”元组与不同的名称联系起来：颜色模型。
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}