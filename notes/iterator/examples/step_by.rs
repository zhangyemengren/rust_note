/// 返回一个迭代器，该迭代器步进n个元素。
fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    let mut iter = arr.iter().step_by(2);
    // 总是先返回第一个元素
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}
