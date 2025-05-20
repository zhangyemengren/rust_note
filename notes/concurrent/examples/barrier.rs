/// 屏障使多个线程能够同步到达后再开始
/// 同步后可以继续同步 如下方例子 可以进行第二轮同步
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

fn main() {
    let thread_count = 3;
    let barrier = Arc::new(Barrier::new(thread_count));

    let mut handles = vec![];

    for i in 0..thread_count {
        let barrier_clone = barrier.clone();
        let handle = std::thread::spawn(move || {
            for round in 0..2 {
                println!("线程{}, 第{}轮, 等待", i, round);
                thread::sleep(Duration::from_millis(500));

                barrier_clone.wait();

                println!("线程{}, 第{}轮, 继续执行", i, round);
                thread::sleep(Duration::from_millis(500));
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("执行结束")
}
