/// 因为变量负责释放自己的资源，所以资源只能有一个所有者。这也可以防止资源被多次释放。请注意，并非所有变量都拥有资源（例如引用）。
/// 当进行赋值（let x = y）或按值传递函数参数（foo(x)）时，资源的所有权将被转移。在 Rust 语言中，这被称为移动。
/// 资源移动后，原所有者将无法再使用。这可以避免创建悬空指针。

// 该函数获取堆分配内存的所有权
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c` 被销毁并释放内存
}

fn main() {
    // _Stack_ 分配的整数
    let x = 5u32;

    // *Copy* `x` into `y` - 不移动任何资源
    let y = x;

    // 两个值可以独立使用
    println!("x is {}, and y is {}", x, y);

    // `a` 是指向 _heap_ 分配的整数的指针
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    let b = a;
    // “a”的指针地址（而不是数据）被复制到“b”中。
    // 两者现在都是指向同一堆分配数据的指针，但“b”现在拥有它。

    // 错误！ `a` 无法再访问数据，因为它不再拥有堆内存
    // println!("a contains: {}", a);
    // TODO ^ Try uncommenting this line

    // 该函数从“b”获取堆分配内存的所有权
    destroy_box(b);

    // 由于此时堆内存已被释放，因此此操作将导致引用已释放的内存，但编译器禁止这样做
    // 错误！与之前的错误原因相同
    // println!("b contains: {}", b);
    // TODO ^ Try uncommenting this line
}
