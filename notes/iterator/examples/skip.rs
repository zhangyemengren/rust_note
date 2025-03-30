/// 创建一个迭代器，该迭代器根据一个谓词跳过元素。
/// skip_while()接受一个闭包作为参数。它将在迭代器的每个元素上调用这个闭包，并忽略元素，直到它返回false。
/// 在返回false之后，skip_while()的任务就结束了，其余的元素被生成。
/// 
/// skip 创建一个迭代器，跳过前n个元素
fn main() {
    // is_negatives是const fn 需显示声明i32类型
    let a = [-2i32, -1, 0, 1, -2];
    let iter = a.iter().skip_while(|x| x.is_negative());
    // 去除-1后停止 所以-2没有被跳过
    println!("{:?}", iter.collect::<Vec<_>>());

    let a = [1, 2, 3, 4, 5];
    let iter = a.iter().skip(2);
    println!("{:?}", iter.collect::<Vec<_>>());
}