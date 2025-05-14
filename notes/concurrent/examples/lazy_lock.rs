use std::sync::{LazyLock, OnceLock};
use std::thread;

/// OnceLock LazyLock
/// LazyLock使用更加便捷

static ONCE: OnceLock<i32> = OnceLock::new();
static LAZY: LazyLock<i32> = LazyLock::new(|| 200);

fn main() {
    assert!(ONCE.get().is_none());

    thread::spawn(|| {
        let value = ONCE.get_or_init(|| 100);
        println!("OnceLock值初始化为:{}", value);
    })
    .join()
    .unwrap();

    thread::spawn(|| {
        // 惰性初始化 运行至此时 才会执行LazyLock new的逻辑
        let value = *LAZY;
        println!("获取到LazyLock植为:{}", value);
    })
    .join()
    .unwrap();

    assert_eq!(ONCE.get().unwrap(), &100);
}
