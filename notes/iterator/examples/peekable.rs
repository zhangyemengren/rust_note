/// 创建一个迭代器，该迭代器可以使用peek和peek_mut方法在不消耗迭代器的情况下查看其下一个元素。
/// 请注意，当首次调用peek或peek_mut时，底层迭代器仍会前进：为了检索下一个元素，会在底层迭代器上调用next。
fn main() {
    let arr = [1, 2, 3];
    let mut iter = arr.iter().peekable();
    println!("{:?}", iter.peek());
    // peek 返回引用 我们可以多次调用 peek ()，迭代器不会前进。
    assert_eq!(iter.peek(), Some(&&1));
    println!("{:?}", iter.peek());
    println!("{:?}", iter.next());
    assert_eq!(iter.next(), Some(&2));
    println!("{:?}", iter.next());
    // 结束都是None
    println!("{:?}", iter.peek());
    println!("{:?}", iter.next());

    // 使用peek_mut在不推进迭代器的情况下修改下一个项：
    let arr = [1, 2, 3];
    let mut iter = arr.iter().peekable();
    assert_eq!(iter.peek_mut(), Some(&mut &1));
    // 多次修改第一个元素 不会前进
    *iter.peek_mut().unwrap() = &99;
    *iter.peek_mut().unwrap() = &98;
    println!("{:?}", iter.collect::<Vec<_>>());
}

