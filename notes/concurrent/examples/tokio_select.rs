use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

#[tokio::main]
async fn main() {
    let mut count = 0;
    loop {
        // 执行顺序乱序
        tokio::select! {
            _ = async { }, if count < 1 => {
                println!("count < 1 为 {}", count);
                count += 1;
            }
            _ = async { }, if count < 2 => {
                println!("count < 2 为 {}", count);
                count += 1;
            }
            _ = async { }, if count < 3 => {
                println!("count < 3 为 {}", count);
                count += 1;
            }
            _ = async { }, if count < 4 => {
                println!("count < 4 为 {}", count);
                count += 1;
            }
            else => {
                break;
            }
        }
    }

    let count = Arc::new(AtomicI32::new(0));
    let mut arr = vec![];
    loop {
        tokio::select! {
            biased; // 保持 biased 轮询顺序

            // 分支 1: count < 3 时执行
            v = async {
                let count_clone = Arc::clone(&count); // 在 async 块内部克隆
                count_clone.fetch_add(1, Ordering::SeqCst); // 原子地加 1
                count_clone.load(Ordering::SeqCst)         // 加载并返回新值 (i32)
            }, if count.load(Ordering::SeqCst) < 1 => {
                println!("Branch 1 selected. count was < 1. New value: {}", v);
                arr.push(v); // v 是 async 块返回的 i32 值
            }

            // 分支 2: count < 2 时执行 (注意这个条件比上一个更严格)
            v = async {
                let count_clone = Arc::clone(&count);
                count_clone.fetch_add(1, Ordering::SeqCst);
                count_clone.load(Ordering::SeqCst)
            }, if count.load(Ordering::SeqCst) < 2 => {
                println!("Branch 2 selected. count was < 2. New value: {}", v); // 添加日志
                arr.push(v);
            }

            // 分支 3: count < 3 时执行 (与分支 1 条件相同)
            v = async {
                let count_clone = Arc::clone(&count);
                count_clone.fetch_add(1, Ordering::SeqCst);
                count_clone.load(Ordering::SeqCst)
            }, if count.load(Ordering::SeqCst) < 3 => {
                 println!("Branch 3 selected. count was < 3. New value: {}", v); // 添加日志
                arr.push(v);
            }

            // 分支 4: count < 4 时执行
            v = async {
                let count_clone = Arc::clone(&count);
                count_clone.fetch_add(1, Ordering::SeqCst);
                count_clone.load(Ordering::SeqCst)
            }, if count.load(Ordering::SeqCst) < 4 => {
                 println!("Branch 4 selected. count was < 4. New value: {}", v);
                arr.push(v);
            }

            // 分支 5: count < 5 时执行
            v = async {
                let count_clone = Arc::clone(&count);
                count_clone.fetch_add(1, Ordering::SeqCst);
                count_clone.load(Ordering::SeqCst)
            }, if count.load(Ordering::SeqCst) < 5 => {
                println!("Branch 5 selected. count was < 5. New value: {}", v);
                arr.push(v);
            }

            // 如果以上条件都不满足 (count >= 5)，则退出循环
            else => {
                println!("No branches selected or count >= 5. Breaking loop.");
                break;
            }
        }
    }
    println!("arr 是按顺序的{:?}", arr);
}
