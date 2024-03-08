// 阻止强制转换导致溢出的警告
#![allow(overflowing_literals)]

/// Rust 不提供原始类型之间的隐式类型转换（强制 coercion）。但是，可以使用 as 关键字执行显式类型转换（强制转换 casting）。
/// 整数类型之间的转换规则通常遵循 C 约定，除非 C 具有未定义的行为。 Rust 中明确定义了整型类型之间所有强制转换的行为。
/// 类型推断引擎非常智能。它不仅仅在初始化期间查看值表达式的类型。它还研究了之后如何使用变量来推断其类型。

// type语句可用于为现有类型指定新名称。类型必须有UpperCamelCase名称，否则编译器会发出警告。
// 别名的主要用途是减少样板代码
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;
fn main() {
    // `NanoSecond` = `Inch` = `U64` = `u64`.
    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    // 请注意，类型别名aliases不提供任何额外的类型安全性，因为别名不是新类型
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
    let decimal = 65.4321_f32;

    // Error! 无隐式转换
    // let integer: u8 = decimal;
    // FIXME ^ Comment out this line

    // 显示转换
    let integer = decimal as u8;
    let character = integer as char;

    // Error! 转换规则存在限制。
    // float 不能直接转换为 char。
    // let character = decimal as char;
    // FIXME ^ Comment out this line

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 当将任何值转换为无符号类型 T 时，T::MAX + 1 增加或减少，直到该值符合新类型

    // 1000已经符合u16类型
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // 在底层，前 8 个最低有效位 (LSB) 被保留，
    // 而最高有效位 (MSB) 的其余部分则被截断。
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // 对于正数，这与模数相同
    println!("1000 mod 256 is : {}", 1000 % 256);

    // 当转换为有符号类型时，（按位）结果与第一次转换为相应的无符号类型相同。如果该值的最高有效位为 1，则该值为负数。

    // 当然，除非它不进行转换也已经适符合。
    println!(" 128 as a i16 is: {}", 128 as i16);

    // 128 as u8 -> 128, 其8位二进制补码表示的值为：
    println!(" 128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // 8 位二进制补码表示中的 232 的值为 -24， 由于最高有效位为 1，因此该值为负数。计算负数的补码
    // tips: 正数的补码：
    // 正数的补码就是其二进制表示本身。例如，十进制数 5 的二进制表示是 0101，它的补码仍然是 0101。
    // 负数的补码：
    // 首先，将负数的绝对值表示为二进制数。
    // 然后，取其二进制数的反码，即将 0 变为 1，将 1 变为 0。
    // 最后，将反码加 1，得到负数的补码。
    // 232 -> 1110 1000 -> 0001 0111 -> 0001 1000 -> 24 -> -24
    println!(" 232 as a i8 is : {}", 232 as i8);

    // 从 Rust 1.45 开始，`as` 关键字执行*饱和转换*
    // 从 float 转换为 int 时。如果浮点值超过 上限或者小于下限，返回值 将等于跨越的界限。

    // 300.0 as u8 is 255
    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("   nan as u8 is : {}", f32::NAN as u8);

    // 上面的这种行为会产生少量的运行时间成本，但可以通过使用不安全的方法避免
    // 代价是结果可能会溢出 返回**不合理的值(unsound values)**。
    // 明智地使用这些方法：
    unsafe {
        // 300.0 as u8 is 44 (-> 300 - 256 = 44)
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156(-> -100 + 256 = 156)
        println!(
            "-100.0 as u8 is : {}",
            (-100.0_f32).to_int_unchecked::<u8>()
        );
        // nan as u8 is 0
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
