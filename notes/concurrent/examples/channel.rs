/// 同步 异步通道
///
use std::sync::mpsc::{channel, sync_channel};
use std::thread;
use std::time::Duration;

fn main() {
    // 异步通道
    let (sc, rc) = channel();

    thread::spawn(move || {
        sc.send(100).unwrap();
    });

    // 阻塞等待接收值
    println!("收到的值为:{}", rc.recv().unwrap());

    // 同步通道 需要缓冲区
    let (sc, rc) = sync_channel(1);
    // 缓冲区1 正常发送
    sc.send(200).unwrap();
    // 超出缓冲区 阻塞直到前一个被接收
    thread::spawn(move || {
        sc.send(300).unwrap();
    });
    thread::sleep(Duration::from_secs(1));
    println!("同步通道接收第一个消息:{}", rc.recv().unwrap());
    println!("同步通道接收第二个消息:{}", rc.recv().unwrap());
}
