use std::thread::{self, sleep};
use std::time::Duration;

/// yield_now 主动让出CPU 类似异步中的await
fn main() {
    println!("主线程开始执行");

    let handle1 = thread::spawn(|| {
        println!("线程1开始执行");
        // 添加一些计算工作
        for i in 1..5 {
            println!("线程1: 计算 {}", i);
            // 每次迭代后尝试让出CPU
            thread::yield_now();
        }
        println!("线程1完成");
    });

    let handle2 = thread::spawn(|| {
        println!("线程2开始执行");
        // 添加一些计算工作
        for i in 1..5 {
            println!("线程2: 计算 {}", i);
            // 每次迭代后尝试让出CPU
            thread::yield_now();
        }
        println!("线程2完成");
    });

    let handle3 = thread::spawn(|| {
        println!("线程3开始执行");
        sleep(Duration::from_millis(100));
        println!("线程3完成");
    });

    // 主线程也参与计算
    for i in 1..3 {
        println!("主线程: 计算 {}", i);
        thread::yield_now();
    }

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();

    println!("所有线程执行完毕");
}
