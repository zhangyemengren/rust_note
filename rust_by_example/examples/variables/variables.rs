/// Rust 通过静态类型提供类型安全。变量绑定可以在声明时进行类型注释。
/// 然而，在大多数情况下，编译器将能够从上下文推断变量的类型，从而大大减少注释负担。
/// 可以使用绑定将值（如字面量）绑定到变量let。

fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // 复制 `an_integer` 到 `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 编译器警告未使用的变量绑定；
    // 这些警告可以 通过在变量名前添加下划线来静音
    let _unused_variable = 3u32;

    // 默认情况下，变量绑定是不可变的，但可以使用mut修饰符覆盖。

    let mut mutable_binding = 1;
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);
    // 变量绑定有一个范围，并且被限制在一个块中。块是用大括号{}括起来的语句的集合。
    {
        // 该绑定仅存在于该块中
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }

    // 允许变量遮蔽
    let shadowed_binding = 1;
    println!("shadowed_binding: {}", shadowed_binding);
    let shadowed_binding = 2;
    println!("shadowed_binding: {}", shadowed_binding);

    // 可以先声明变量绑定，然后再初始化它们。然而，这种形式很少使用，因为它可能导致使用未初始化的变量。
    // 编译器禁止使用未初始化的变量，因为这会导致未定义的行为。
    let a_binding;
    // println!("a_binding: {}", a_binding); error
    {
        let x = 2;
        a_binding = x * x;
    }
    println!("a binding: {}", a_binding);

    // 当数据被同名不可改变地绑定时，它也会冻结。在不可变绑定超出范围之前，无法修改冻结数据：
    let mut _mutable_integer = 7i32;
    {
        // shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // _mutable_integer = 50; // error
        // `_mutable_integer` goes out of scope
    }
    _mutable_integer = 50;
}