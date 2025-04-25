/// tokio task vs future
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

struct MyFuture;

impl Future for MyFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(1)
    }
}

#[tokio::main]
async fn main() {
    let future = MyFuture;
    let result = future.await;
    println!("Future 结果: {}", result);

    let task = tokio::spawn(async { 42 });

    let block_task = tokio::task::spawn_blocking(move || {
        thread::sleep(Duration::from_secs(1));
        return 100;
    })
    .await
    .unwrap();

    let result = task.await.unwrap();
    println!("Tokio::task 结果: {}", result);
    println!("阻塞任务结果: {}", block_task);
}
