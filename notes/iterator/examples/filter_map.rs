/// 创建一个既过滤又映射的迭代器。返回的迭代器仅生成提供的闭包返回Some(value)的value
fn main() {
    let arr = [1, 2, 3, 4, 5];
    // 只保留偶数并将它们乘以2
    let result = arr.iter()
        .filter_map(|&x| if x % 2 == 0 { Some(x * 2) } else { None })
        .collect::<Vec<_>>();
    println!("{:?}", result);
}
