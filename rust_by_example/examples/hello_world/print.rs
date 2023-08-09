/// 打印是由std::fmt定义一系列macros 来处理的，其中一些包括：
/// format!：将格式化文本写入String
/// print!：与format!相同，但文本被打印到控制台（io::stdout）。
/// println!: 与print!相同，但附加了换行符。
/// eprint!：与print!相同，但文本被打印到标准错误 (io::stderr)。
/// eprintln!: 与eprint!相同，但附加了换行符。
/// 全部以相同的方式解析文本。另外，Rust 在编译时检查格式的正确性。

fn main() {
    // 一般来说，`{}`会自动替换为任何
    // 参数。这些将被字符串化。
    println!("{} days", 31);

    // 可以使用位置参数。在“{}”内指定一个整数
    // 确定将替换哪个附加参数。
    // 参数在紧接在格式字符串之后的 0 处开始。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以与命名参数一样
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // 在`:`之后 通过指定格式字符可以调用不同的格式
    println!("Base 10:               {}", 69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

    // 您可以将文本按指定宽度右对齐。这会
    // 输出“1”。 （四个空格和一个“1”，总宽度为 5。）
    println!("{number:>5}", number = 1);

    // 您可以用额外的零填充数字
    println!("{number:0>5}", number = 1); // 00001
                                          // 并通过翻转标志向左调整。这将输出“10000”。
    println!("{number:x<5}", number = 1); // 1xxxx

    // 您可以通过附加“$”在格式说明符中使用命名参数。
    println!("{number:0>width$}", number = 1, width = 5);

    // Rust 甚至会检查以确保使用了正确数量的参数。
    // println!("My name is {0}, {1} {0}", "Bond");
    // FIXME 🔼 Add the missing argument: "James"  after 🔽
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // 只有实现 fmt::Display 的类型才能使用 `{}` 进行格式化。
    // 用户-定义的类型默认不实现 fmt::Display。

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // 这不会编译，因为 `Structure` 没有实现 fmt::Display
    // println!("这个结构`{}`不会打印...", Structure(3));
    // TODO 🔼 Try uncommenting this line (error: `Structure` doesn't implement `std::fmt::Display`)

    // 对于 Rust 1.58 及以上版本，您可以直接从周围变量 中捕获参数
    // 中捕获参数。这将输出 4 个空格和一个“1”。
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    //std::fmt包含许多traits控制文本显示的内容。下面列出了两个重要的基本形式：
    // fmt::Debug：使用{:?}标记。设置文本格式以用于调试目的。
    // fmt::Display：使用{}标记。以更优雅、用户友好的方式设置文本格式。
}
