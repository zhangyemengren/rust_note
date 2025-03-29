// 推进迭代器并返回下一个值。当迭代完成时返回None。
//各个迭代器的实现可以选择恢复迭代，因此再次调用next()可能会在某个时候再次开始返回Some(Item)，也可能不会。
fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    let mut iter = arr.iter();
    // iter.size_hint按元组返回迭代器下限和上限
    // 下限等于len(需实现ExactSizeIterator) 上限如果是None意味着要么没有已知的上界，要么上界大于usize。
    println!("{:?} {:?}", iter.len(), iter.size_hint());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    // 为空时返回None 可继续迭代
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?} {:?}", iter.len(), iter.size_hint());
    // 无上限迭代器
    let mut iter = 0..;
    println!("{:?}", iter.size_hint());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.size_hint());
}
