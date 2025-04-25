use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// 基于线程挂起机制的阻塞队列实现
/// 使用 thread::park() 和 unpark() 实现线程等待和唤醒
struct BlockingQueue<T> {
    queue: Arc<Mutex<Vec<T>>>, // 存储队列数据的容器
    available: AtomicBool,     // 标记队列中是否有可用的数据
    thread: thread::Thread,    // 消费者线程的句柄，用于唤醒操作
}

impl<T> BlockingQueue<T> {
    /// 创建新的阻塞队列实例
    fn new() -> Self {
        BlockingQueue {
            queue: Arc::new(Mutex::new(Vec::new())),
            available: AtomicBool::new(false),
            thread: thread::current(), // 获取当前线程的句柄
        }
    }

    /// 向队列中添加元素并唤醒可能等待中的消费者线程
    fn push(&self, item: T) {
        let mut queue = self.queue.lock().unwrap(); // 获取队列的互斥锁
        queue.push(item); // 将元素添加到队列
        self.available.store(true, Ordering::SeqCst); // 设置数据可用标志
        self.thread.unpark(); // 唤醒可能被挂起的消费者线程
    }

    /// 从队列中取出元素，如果队列为空则阻塞当前线程直到有新元素
    fn pop(&self) -> T {
        loop {
            let mut queue = self.queue.lock().unwrap(); // 获取队列的互斥锁
            if let Some(item) = queue.pop() {
                // 尝试取出队列中的元素
                return item; // 如果成功则返回元素
            }
            drop(queue); // 释放锁，以便生产者能添加元素
            self.available.store(false, Ordering::SeqCst); // 标记队列为空
            thread::park(); // 挂起当前线程，等待被唤醒
            // 当线程被唤醒后，会继续循环尝试获取元素
        }
    }
}

fn main() {
    // 创建阻塞队列并在两个线程间共享
    let queue = Arc::new(BlockingQueue::new());
    let queue2 = queue.clone();

    // 生产者线程
    thread::spawn(move || {
        for i in 0..10 {
            queue2.push(i); // 生产元素
            println!("生产者生产: {}", i);
            thread::sleep(Duration::from_millis(100)); // 模拟生产过程的耗时
        }
    });

    // 消费者线程（主线程）
    for _ in 0..10 {
        let item = queue.pop(); // 消费元素，如果队列为空会阻塞
        println!("消费者消费: {}", item);
    }
}
