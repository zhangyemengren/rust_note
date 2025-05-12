use std::any::type_name;
use std::sync::{Arc, Mutex};
use std::thread;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn main() {
    // 锁中毒
    let m = Arc::new(Mutex::new(0));
    let m2 = Arc::clone(&m);

    let handle = thread::spawn(move || {
        let mut guard = m2.lock().unwrap();
        *guard += 1;
        panic!("触发panic!");
    });

    let _ = handle.join();

    assert_eq!(m.is_poisoned(), true);

    let x = m.lock().unwrap_or_else(|mut e| {
        // 解引用链
        println!("e的类型是{:?}", type_of(&e));
        println!("e.get_mut()的类型是{:?}", type_of(&e.get_mut()));
        println!("*e.get_mut()的类型是{:?}", type_of(&*e.get_mut()));
        println!("**e.get_mut()的类型是{:?}", type_of(&**e.get_mut()));
        **e.get_mut() += 1;
        m.clear_poison();
        e.into_inner()
    });
    assert_eq!(m.is_poisoned(), false);
    println!("中毒锁清理后返回的新锁的值为{}", x);

    // into_inner消耗锁
    let mutex = Mutex::new(0);
    let x = mutex.into_inner();
    assert_eq!(x.unwrap(), 0);
}
