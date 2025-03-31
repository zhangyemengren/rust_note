/// 创建一个迭代器，该迭代器给出当前迭代计数以及下一个值。
/// 返回的迭代器生成对（i，val），其中 i 是当前迭代索引，val 是迭代器返回的值。
fn main() {
    let a = ['a', 'b', 'c'];

    let mut iter = a.iter().enumerate();

    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}
