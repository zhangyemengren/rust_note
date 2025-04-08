use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rayon;

fn main() {
    // 使用rayon库创建一个线程池, 包含4个线程
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    // 共享计数器， 模拟跨线程共享数据
    let counter = Arc::new(Mutex::new(0));
    let counter2 = Arc::clone(&counter);

    // 创建一个闭包作为任务，修改共享的计数器
    let op = move || {
        let mut counter = counter.lock().unwrap();
        *counter += 1;
        println!("线程 {:?} 在执行任务, 数量: {}", thread::current().id(), *counter);
    };

    // 直接使用spawn 启动任务

    pool.spawn(move || {
        println!("spawn 方法执行");
        op();
    });
    println!("spawn不会阻塞");

    // 直接使用scope 启动多个任务（确保所有任务完成后再返回）
    pool.scope(|s| {
       println!("scope 方法执行");
        s.spawn(|_| {
            println!("scope 中spawn 一个任务");
        });
        s.spawn(|_| {println!("scope 中spawn 另一个任务")})
    });
    println!("scope会阻塞 等待所有任务返回");

    // 直接使用scope_fifo 启动多个任务（先发先执行，任务按照提交顺序执行）
    pool.scope_fifo(|s| {
        println!("scope_fifo 方法执行");
        s.spawn_fifo(|_| {
            println!("scope_fifo spawn_fifo任务1 线程{:?}", thread::current().id());
        });
        s.spawn_fifo(|_| {
            println!("scope_fifo spawn_fifo任务2 线程{:?}", thread::current().id());
        });
    });

    // in_place_scope_fifo 启动多个任务（先发先执行，但在同一线程内顺序执行）
    pool.in_place_scope_fifo(|s| {
        println!("in_place_scope_fifo 方法执行");
        s.spawn_fifo(|_| {
            println!("in_place_scope_fifo spawn_fifo任务1 线程{:?}", thread::current().id());
        });
        s.spawn_fifo(|_| {
            println!("in_place_scope_fifo spawn_fifo任务2 线程{:?}", thread::current().id());
        });
    });

    //
    pool.spawn_fifo(|| {
        println!("spawn_fifo 方法执行");
    });

    // spawn_broadcast 广播任务到所有线程
    // 特点：非阻塞操作，不等待任务完成就返回
    // 适用场景：任务需要在所有线程执行，但不需要等待结果的场景
    pool.spawn_broadcast(|_| {
        println!("spawn_broadcast 方法执行  线程{:?}", thread::current().id());
    });

    // broadcast 广播任务到所有线程
    // 特点：阻塞操作，会等待所有任务完成后才返回
    // 适用场景：需要确保所有线程都完成特定任务后再继续的场景
    pool.broadcast(|_| {
        println!("broadcast 方法执行  线程{:?}", thread::current().id());
    });

    // 等待
    thread::sleep(Duration::from_secs(1));
}