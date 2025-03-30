/// 接受两个迭代器，并按顺序对两个迭代器创建一个新的迭代器。
/// chain()将返回一个新的迭代器，它首先遍历第一个迭代器的值，然后遍历第二个迭代器的值。
/// 换句话说，它将两个迭代器链接在一起，形成一个链。通常用于将单个值适配到其他类型的迭代链中。
fn main() {
    let a1 = vec![1, 2, 3];
    let a2 = vec![4];
    let a3 = vec![5, 6, 7];
    let mut iter = a1.iter().chain(a2.iter()).chain(a3.iter());
    while let Some(v) = iter.next() {
        println!("{}", v);
    }
}
