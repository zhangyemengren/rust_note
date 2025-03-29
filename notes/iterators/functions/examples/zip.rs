/// 将两个迭代器压缩在一起，成为一个包含成对元素的单一迭代器。
/// zip()返回一个新的迭代器，该迭代器将遍历另外两个迭代器，返回一个元组，其中第一个元素来自第一个迭代器，第二个元素来自第二个迭代器。
/// 换句话说，它将两个迭代器压缩在一起，成为一个单一的迭代器。如果任一迭代器返回None，则压缩后的迭代器的next方法将返回None。
/// 如果压缩后的迭代器没有更多元素可返回，那么每次进一步尝试推进它时，将首先最多尝试推进第一个迭代器一次，如果它仍然产生一个项目，则最多尝试推进第二个迭代器一次。
fn main() {
    let a1 = vec![1, 2];
    let a2 = vec![4, 5, 6];
    let mut iter = a1.iter().zip(a2.iter());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    // zip()常被用于将一个无限迭代器与一个有限迭代器进行组合。这是可行的，因为有限迭代器最终会返回None，从而结束组合操作。与(0..)进行组合看起来很像enumerate(带下标的iter)：
    let enumerate: Vec<_> = "foo".chars().enumerate().collect();
    let ziper: Vec<_> = (0..).zip("foo".chars()).collect();
    println!("{:?}", enumerate[0]);
    println!("{:?}", ziper[0]);
    // 如果两个迭代器具有大致相同的语法，那么使用zip方法可能更具可读性。
    let a1 = vec![1, 2, 3];
    let a2 = vec![4, 5, 6];
    let mut zipped = std::iter::zip(a1.into_iter().map(|x| x * x), a2.into_iter().map(|x| x * x));
    println!("{:?}", zipped.next());
    println!("{:?}", zipped.next());
    // 类似于
    let a1 = vec![1, 2, 3];
    let a2 = vec![4, 5, 6];
    let mut zipped = a1
        .into_iter()
        .map(|x| x * x)
        .zip(a2.into_iter().map(|x| x * x));
    println!("{:?}", zipped.next());
    println!("{:?}", zipped.next());
}
