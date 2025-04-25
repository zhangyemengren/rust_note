use std::{thread, time::Duration};

/// thread::park  将阻塞当前线程，直到线程的许可可用。此时它以原子操作的使用许可。thread::park_timeout执行相同的操作，但允许指定阻止线程的最长时间。
/// 和  sleep  不同，它可以还未到超时的时候就被唤醒。thread.upark  方法以原子方式使许可可用（如果尚未可用）
/// 许可 (Permit)：每个线程都关联着一个许可。unpark 给予许可，park 消耗许可。
/// 许可最多只有一个，重复unpark不会累积多个许可。非阻塞的 unpark：如果线程在调用 unpark 时没有被 park，则许可会被保留，直到线程调用 park 时立即使用，避免阻塞。
/// 这与传统的 wait/notify 机制不同，后者要求 notify 必须在 wait 之后调用才能生效。线程安全性：park 和 unpark 是线程安全的，可以在不同的线程中调用。
fn thread_park() {
    let handle = thread::spawn(|| {
        thread::park();
        println!("线程被唤醒");
    });

    println!("主线程等待1秒");
    thread::sleep(Duration::from_secs(1));

    handle.thread().unpark();

    handle.join().unwrap();
}

fn thread_park2() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_secs(1));
        // 提前park 线程 不会阻塞
        thread::park();
        println!("线程被唤醒");
    });

    // 注释此代码 程序会挂起
    handle.thread().unpark();

    handle.join().unwrap();
}
fn main() {
    thread_park();
    thread_park2();
}
