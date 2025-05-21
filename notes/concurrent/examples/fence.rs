/// fence围栏
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering, fence};
use std::thread;

// 全局原子变量用于线程间通信
// DATA 用于存储共享数据
static DATA: AtomicI32 = AtomicI32::new(0);
// READY 作为标志，表示数据是否准备好
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    // 生产者线程
    let producer_handle = thread::spawn(|| {
        // 1. 存储数据，使用 Relaxed 排序，因为后续会有显式的 fence
        DATA.store(42, Ordering::Relaxed);
        println!("Producer: Stored DATA = 42 (Relaxed)");

        // 2. 插入 Release 屏障
        // Release 屏障确保在此屏障之前的所有写操作（如此处的 DATA.store）
        // 对于之后任何读取 READY 标志（并且该读取操作带有 Acquire 语义或之后有 Acquire 屏障）
        // 的其他线程都是可见的。
        // 它与 Acquire 屏障或 Acquire 操作同步。
        fence(Ordering::Release);
        println!("Producer: Issued Release fence");

        // 3. 设置 READY 标志为 true，表示数据已准备好
        // 使用 Relaxed 排序，因为 Release 屏障已经保证了 DATA 的可见性。
        // 另一种选择是 READY.store(true, Ordering::Release)，这本身就带有 Release 语义，
        // 可以替代上面单独的 fence(Ordering::Release)，如果这是此线程最后的相关操作。
        READY.store(true, Ordering::Relaxed);
        println!("Producer: Set READY = true (Relaxed)");
    });

    // 消费者线程
    let consumer_handle = thread::spawn(|| {
        // 1. 等待 READY 标志变为 true
        // 使用 Relaxed 排序轮询，因为后续会有显式的 Acquire 屏障。
        // 另一种选择是 while !READY.load(Ordering::Acquire) { ... }，
        // 这本身就带有 Acquire 语义，可以替代下面单独的 fence(Ordering::Acquire)
        // 来保证后续读取的顺序。
        print!("Consumer: Waiting for READY...");
        while !READY.load(Ordering::Relaxed) {
            // 短暂休眠以避免过度消耗 CPU，实际应用中可能有更优的同步方式
            thread::yield_now(); // 让出CPU给其他线程
        }
        println!(" done.");

        // 2. 插入 Acquire 屏障
        // Acquire 屏障确保在此屏障之后的所有读操作（如此处的 DATA.load）
        // 能够观察到与 Release 屏障（或 Release 操作）同步的线程所做的写操作。
        // 也就是说，一旦我们读到 READY 为 true (该写入之前有一个 Release 屏障)，
        // 那么在此 Acquire 屏障之后，DATA 的写入（值为42）保证是可见的。
        fence(Ordering::Acquire);
        println!("Consumer: Issued Acquire fence");

        // 3. 读取数据
        // 使用 Relaxed 排序，因为 Acquire 屏障已经保证了数据的可见性。
        let value = DATA.load(Ordering::Relaxed);
        println!("Consumer: Read DATA = {} (Relaxed)", value);

        // 4. 断言读取到的数据是生产者写入的值
        assert_eq!(value, 42, "Consumer: Data should be 42");
        println!("Consumer: Assertion passed (DATA == 42).");
    });

    // 等待两个线程执行完毕
    producer_handle.join().expect("Producer thread panicked");
    consumer_handle.join().expect("Consumer thread panicked");

    println!("Main: Example finished successfully.");
}