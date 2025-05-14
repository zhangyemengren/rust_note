use std::sync::Once;
use std::thread;
use std::time::Duration;

static ONCE: Once = Once::new();
static READY: Once = Once::new();

fn main() {
    let handle = thread::spawn(move || {
        ONCE.call_once(|| panic!("panic"));
    });
    assert_eq!(handle.join().is_err(), true);
    // call_once panic后 会中毒 后续每一个调用都是panic
    let handle = thread::spawn(move || {
        ONCE.call_once(|| {});
    });
    assert_eq!(handle.join().is_err(), true);
    // 调用 call_once_force 会执行初始化函数，并重置中毒状态
    let handle = thread::spawn(move || {
        ONCE.call_once_force(|state| {
            println!("是否中毒为中毒状态:{}", state.is_poisoned());
        });
    });
    assert_eq!(handle.join().is_err(), false);
    // 一旦成功调用一次，就不会再传播中毒状态 此时不会再panic
    ONCE.call_once(|| {
        // 不再中毒 再次调用是空操作 不会再执行（同once正常逻辑）
        println!("ok");
    });

    // wait
    thread::spawn(move || {
        READY.wait();
        println!("使用wait 会阻塞直到Once完成")
    });
    READY.call_once(|| {
        println!("执行第二个Once初始化");
        thread::sleep(Duration::from_secs(1));
    });

    println!("程序结束");
}
