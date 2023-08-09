///  任何程序都需要注释，Rust 支持几种不同的类型：
/// 1 编译器会忽略的 常规注释：
/// // Line comments which go to the end of the line.
/// /* Block comments which go to the closing delimiter. */
/// 2 文档注释被解析为 HTML 库文档：
/// /// Generate library docs for the following item.
/// //! Generate library docs for the enclosing item.

fn main() {
    // 这是行注释的示例。
    // 行首有两个斜杠。
    // 编译器不会读取这些之后写入的任何内容。

    // println!("Hello, world!");

    // 运行。查看 现在尝试删除两个斜杠，然后再次运行。

    /*
     * 这是另一种类型的注释，块注释。一般来说，
     * 行注释是推荐的注释风格。但屏蔽评论
     * 对于暂时禁用代码块非常有用。
     * /* 块注释可以 /* 嵌套, */ */ 所以只需要几个
     * 通过按键注释掉 main() 函数中的所有内容。
     * /*/*/* 自己尝试一下！ */*/*/
     */

    /*
    注意：前一列“*”完全是为了样式。
    没有实际需要。
    */

    // 您可以使用块注释更轻松地操作表达式
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}