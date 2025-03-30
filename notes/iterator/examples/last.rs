/// 消耗迭代器，返回最后一个元素。此方法将评估迭代器，直到它返回None。
/// 在执行此操作时，它会跟踪当前元素。在返回None后，last()将返回它看到的最后一个元素。
fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    println!("{:?}", arr.iter().last());
    let arr: Vec<i32> = vec![];
    println!("{:?}", arr.iter().last());
}
