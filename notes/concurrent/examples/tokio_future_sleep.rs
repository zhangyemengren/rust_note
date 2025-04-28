use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::{sleep, Sleep};

struct Delay {
    sleep: Sleep,
    message: String,
    _pin: PhantomPinned,
}

impl Delay {
    fn new(duration: Duration, message: String) -> Self {
        Self {
            sleep: sleep(duration),
            message,
            _pin: PhantomPinned,
        }
    }
    fn get_pinned_sleep(self: Pin<&mut Self>) -> Pin<&mut Sleep> {
        unsafe {
            let this = Pin::get_unchecked_mut(self);
            Pin::new_unchecked(&mut this.sleep)
        }
    }
}

impl Future for Delay {
    type Output = String;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let sleep = self.as_mut().get_pinned_sleep();
        match sleep.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(_) => Poll::Ready(self.message.clone()),
        }
    }
}

#[tokio::main]
async fn main() {
    let delay1 = Delay::new(Duration::from_secs(2), "第一个延迟完成".to_string());
    let delay2 = Delay::new(Duration::from_secs(1), "第二个延迟完成".to_string());
    tokio::join!(
        async {
            let res1 = delay1.await;
            println!("res1结果: {}", res1);
        },
        async {
            let res2 = delay2.await;
            println!("res2结果: {}", res2);
        }
    );
    println!("完成");
}
