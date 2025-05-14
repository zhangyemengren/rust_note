use std::sync::RwLock;
use std::thread;

fn main() {
    let lock = RwLock::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            let mut w = lock.write().unwrap();
            *w += 1;
        });
        s.spawn(|| {
            let r = lock.read().unwrap();
            println!("{:?}", r);
        });
        s.spawn(|| {
            let r = lock.read().unwrap();
            println!("{:?}", r);
        });
    })
}
