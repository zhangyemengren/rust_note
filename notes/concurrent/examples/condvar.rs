/// 条件变量表示阻塞线程的能力，使其在等待事件发生时不消耗 CPU 时间。
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(""), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut start = lock.lock().unwrap();
        *start = "改变值";
        println!("发出通知");
        cvar.notify_all();
    });
    let (lock, cvar) = &*pair;
    let mut start = lock.lock().unwrap();

    // 此时会阻塞等待
    while start.is_empty() {
        println!("1");
        start = cvar.wait(start).unwrap();
    }

    println!("执行结束, start值为:{:?}", start);
}
