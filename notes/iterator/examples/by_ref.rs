/// 借用一个迭代器，而不是消耗它。这对于在仍然保留原始迭代器所有权的同时应用迭代器适配器很有用。
fn main() {
    let mut before = vec![1, 2, 3, 4].into_iter();
    let after: Vec<_> = before.by_ref().take(2).collect();
    println!("before{:?}", before.collect::<Vec<_>>());
    println!("after{:?}", after);

    #[allow(unused_mut)]
    let mut before = vec![1, 2, 3, 4].into_iter();
    // 不使用by_ref 原vec被消耗 无法再使用
    let after: Vec<_> = before.skip(2).collect();
    // println!("before{:?}", before.collect::<Vec<_>>());
    println!("after{:?}", after);
}
