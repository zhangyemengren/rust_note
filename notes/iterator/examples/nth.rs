/// 返回迭代器的第n个元素。
/// 像大多数索引操作一样，计数从零开始，所以nth(0)返回第一个值，nth(1)返回第二个值，依此类推。
/// 请注意，所有前面的元素以及返回的元素都将从迭代器中消耗掉。这意味着前面的元素将被丢弃，并且在同一个迭代器上多次调用nth(0)将返回不同的元素。
/// 如果n大于或等于迭代器的长度，nth()将返回None。
fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    let mut iter = arr.iter();
    println!("{:?}", iter.nth(0));
    println!("{:?}", iter.nth(0));
    println!("{:?}", iter.nth(2));
    println!("{:?}", iter.nth(2));
}
