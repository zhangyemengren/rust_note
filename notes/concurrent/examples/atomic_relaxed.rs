use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// 定义：仅保证操作的原子性，没有额外的内存排序约束。
/// 适用场景：当只需要确保操作本身不被其他线程干扰，但不关心内存可见性或顺序时使用。例如，线程本地计数器的递增。
/// 特性：不提供任何跨线程的同步保证，适合性能敏感但不要求一致性的场景。
///
fn main() {
    // 原子计数器
    let counter = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];

    for _ in 0..10{
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100{
                // fetch_add 返回之前的值，并添加第一个参数到原子变量 
                // Relaxed 表示我们只关心这个操作的原子性，不关心与其他内存操作的顺序
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles{
        handle.join().unwrap();
    }
    println!("最终计数器为:{}", counter.load(Ordering::Relaxed));
}