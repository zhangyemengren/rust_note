/// panic处理钩子
use std::panic;
fn main() {
    panic::set_hook(Box::new(|err| {
        println!("捕获{}", err);
    }));
    panic!("测试");
}