use std::thread::sleep;
use std::time::Duration;
use tokio::task;

#[tokio::main]
async fn main() {
    // 阻塞函数
    let join  = task::spawn_blocking(|| {
        sleep(Duration::from_secs(1));
        100
    });
    let result = join.await;
    println!("阻塞运行完成{:?}", result);
    
    //与 spawn_blocking 不同的是，block_in_place 通过将当前工作线程转换为阻塞线程来工作，
    // 并将该线程上运行的其他任务移至另一个工作线程。这可以通过避免上下文切换来提高性能
    let result = task::block_in_place(|| {
        sleep(Duration::from_secs(1));
        100
    });
    println!("block_in_place完成{:?}", result);
}