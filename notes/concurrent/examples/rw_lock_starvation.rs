/// 锁 饥饿
/// 当一个锁长期占用 导致另一个线程无法获取锁 会发生饥饿

use std::sync::{Arc,RwLock};
use std::thread;
use std::time::Duration;
fn main() {
    let lock = Arc::new(RwLock::new(0));

    for _ in 0..10 {
        let lock = lock.clone();
        thread::spawn(move || {
            loop{
                let r = lock.read().unwrap();
                println!("读取锁: {:?}", r);
                thread::sleep(Duration::from_millis(10));
            }
        });
    }
    
    thread::sleep(Duration::from_millis(1000));
    
    // 写入锁可能会因为读取锁一直占用而`饿死`
    thread::spawn(move || {
        for i in 0 ..10{
            let mut w = lock.write().unwrap();
            *w += i;
            println!("写入锁:{:?}", w);   
        }
    });
    
    thread::sleep(Duration::from_millis(1000));
}