use num_cpus;
use std::thread;

fn main() {
    // 基础信息
    let current_thread = thread::current();
    println!("当前线程名: {:?}", current_thread.name());
    println!("当前线程ID: {:?}", current_thread.id());

    // 并发度
    println!("并发度: {:?}", thread::available_parallelism().unwrap());

    // 获取CPU核心数 第三方crate
    println!("CPU核心数: {:?}", num_cpus::get());
}
