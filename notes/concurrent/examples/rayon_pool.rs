use std::thread;

fn main() {
    let _ = rayon::ThreadPoolBuilder::new()
        .num_threads(3)
        .thread_name(|i| format!("自定义线程名-{}", i))
        .build_global()
        .unwrap();

    // 只会有3个线程执行任务
    rayon::scope(|s|{
        (0..50).for_each(|i| {
            s.spawn(move |_| {
                println!("循环{} 自定义线程名-{}", i, thread::current().name().unwrap());
            });
        });
    });
}