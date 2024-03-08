/// Rust 提供了对各种原语的访问
/// 1 标量类型
/// 有符号整数：i8、i16、i32、i64、i128 和 isize（指针大小）
/// 无符号整数：u8、u16、u32、u64、u128 和 usize（指针大小）
/// 浮点数：f32,f64
/// char Unicode标量值，如'a'、'α'和'∞'（每个 4 个字节）
/// bool 要么true或者false
/// 单位类型()，其唯一可能的值为空元组：()
/// 尽管单元类型的值是元组，但它不被视为复合类型，因为它不包含多个值。
/// 2 复合类型
/// 数组就像[1, 2, 3]
/// 元组就像(1, true)
fn main() {
    // 变量总是可以带有类型注释。数字还可以通过后缀或默认情况进行注释。
    // 整数默认为i32，浮点数默认为f64。请注意，Rust 还可以从上下文推断类型。
    let logical: bool = true;

    let a_float: f64 = 1.0; // 常规注释
    let an_integer = 5i32; // 后缀注释

    //默认类型
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    // 可以从上下文推断类型
    let mut inferred_type = 12; // i64 类型是来自另一行推断出来的。
    inferred_type = 4294967296i64;

    // 可变变量的值可以更改。
    let mut mutable = 12; // 可变的 `i32`
    mutable = 21;

    // Error! 类型不可变
    // mutable = true;

    // 变量可以通过遮蔽来覆盖
    let mutable = true;
}
