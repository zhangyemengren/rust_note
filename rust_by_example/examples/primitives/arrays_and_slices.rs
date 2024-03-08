/// 数组是存储在连续内存中的相同类型T的对象的集合。
/// 数组是使用方括号[]创建的，其长度在编译时已知，是其类型签名的一部分[T; length]。
/// 切片与数组类似，但它们的长度在编译时未知。
/// 切片是一个两个字的对象；第一个字是指向数据的指针，第二个字是切片的长度。
/// 字大小与 usize 相同，由处理器架构决定，例如 x86-64 上是 64 位。
/// 切片可用于借用数组的一部分并具有类型签名&[T]。
use std::mem;

// This function borrows a slice.
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // 固定大小的数组（类型签名是多余的）。
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 所有元素都可以初始化为相同的值。
    let ys: [i32; 500] = [0; 500];

    // 下标从0开始
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` 返回数组中元素的数量
    println!("Number of elements in array: {}", xs.len());

    // 数组是栈分配的
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // 数组可以自动借用为切片
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // 切片可以指向数组的一部分。
    // 它们的形式为 [starting_index..ending_index]。
    // `starting_index` 是切片中的第一个位置。
    // `ending_index` 比切片中的最后一个位置多 1。
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    // 空切片 `&[]` 的示例
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // 相同但更冗长

    // 可以使用`.get`安全地访问数组，它返回一个
    // `Option`。这可以进行如下所示的匹配，或者
    // 如果你希望程序以一条不错的消息退出而不是愉快地继续，
    // 则可以与“.expect()”一起使用。
    for i in 0..xs.len() + 1 {
        // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // 数组上的越界索引会导致编译时错误。
    // println!("{}", xs[5]);
    // 切片上的越界索引会导致运行时错误。
    // println!("{}", xs[..][5]);
}
