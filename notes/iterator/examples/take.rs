/// take_while创建一个基于谓词生成元素的迭代器。take_while()接受一个闭包作为参数。
/// 它将对迭代器的每个元素调用这个闭包，并在闭包返回true时生成元素。在返回false后，take_while()的任务就结束了，其余的元素将被忽略。
/// 
/// take 创建一个迭代器，生成前n个元素
fn main() {
    // is_negatives是const fn 需显示声明i32类型
    let a = [-2i32, -1, 0, 1, -2];
    let iter = a.iter().take_while(|x| x.is_negative());
    // 去除-1后停止 所以-2没有被跳过
    println!("{:?}", iter.collect::<Vec<_>>());

    let a = [1, 2, 3, 4, 5];
    let iter = a.iter().take(2);
    println!("{:?}", iter.collect::<Vec<_>>());
}