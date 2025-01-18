// 解除注释 尝试运行
// 原因是它foo只有一个生命周期。这意味着：
// func接受具有生命周期的引用'a。
// 由于每个i只能存活一次循环，所以它们不能存活足够长的时间。
// fn foo<'a, T: Fn(&'a i32) -> &'a i32>(func: T) {
//     for i in 0..5 {
//         println!("{}", func(&i));
//     }
// }

// T是一个函数：对于每个生命周期'a，接受&'a i32并返回&'a i32。
// 明确指出界限
fn foo_hrtb<T: for<'a> Fn(&'a i32) -> &'a i32>(f: T) {
    for i in 0..5 {
        println!("{}", f(&i));
    }
}

fn main() {
    // println!("运行不使用HRTB的版本:");
    // foo(|i| i);

    println!("运行使用HRTB的版本:");
    foo_hrtb(|i| i);
}
