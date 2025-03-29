// 消耗迭代器，计算迭代次数并返回该次数。此方法将反复调用next，
//直到遇到None，返回它看到Some的次数。请注意，即使迭代器没有任何元素，也必须至少调用一次next。
fn main() {
    let arr = vec![1, 2, 3];
    let iter = arr.iter();
    println!("{:?}", iter.count());
    // 迭代器被消耗，无法继续使用
    // println!("{:?}", iter);
    let arr: Vec<i32> = vec![];
    let iter = arr.iter();
    println!("{:?}", iter.count());
}
