/// catch_unwind此函数可能无法捕获所有 Rust 恐慌。
/// Rust 恐慌并非总是通过展开实现的，也可以通过中止进程来实现。
/// 此函数仅捕获展开恐慌，而不捕获中止进程的恐慌。
use std::{panic};

fn main() {
    let result1 = panic::catch_unwind(||{
        println!("ok");
    });
    let result2 = panic::catch_unwind(||{
        panic!("error");   
    });
    result1.unwrap();
    result2.unwrap_or_else(|err| {
        println!("捕获错误: {:?}", err);
    });
    println!("程序结束");
}