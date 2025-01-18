// 解除注释 尝试运行
// fn foo<'a, T: Fn(&'a i32) -> &'a i32>(func: T) {
//     for i in 0..5 {
//         println!("{}", func(&i));
//     }
// }

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
