/// 消耗一个迭代器，从中创建两个集合。传递给partition()的谓词可以返回true或false。
/// partition()返回一对，即所有返回true的元素和所有返回false的元素。
fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (even, odd): (Vec<i32>, Vec<i32>) = a.into_iter().partition(|x| x % 2 == 0);
    println!("even: {:?}", even);
    println!("odd: {:?}", odd);
}
