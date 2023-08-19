/// std库中的闭包例子
/// Iterator::find是一个迭代迭代器并搜索满足某些条件的第一个值的函数。
/// 如果没有一个值满足条件，则返回None。它的签名：
/// pub trait Iterator {
///     // 正在迭代的类型。
///     type Item;
///
///     // `find` 采用 `&mut self` 意味着调用者可能是mut借用的，但未消耗。
///     fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
///         // “FnMut”意味着任何捕获的变量最多只能被修改，而不是被消耗。
///         // `&Self::Item` 声明它通过引用将参数传递给闭包。
///         P: FnMut(&Self::Item) -> bool;
/// }

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`.
    let mut iter = vec1.iter();
    // `into_iter()` for vecs yields `i32`.
    let mut into_iter = vec2.into_iter();

    // 在rust中 函数和闭包参数也是一种模式 所以可以使用模式的语法
    // 下面使用了模式的解构引用语法 可参考rust_by_example/examples/flow_control/match_destructuring.rs
    // vecs 的 `iter()` 产生 `&i32`，我们想要引用它的一项，所以我们必须将 `&&i32` 解构为 `i32`
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    // iter.find(|&&x| x == 2) 等同于 iter.find(|x| **x == 2)
    // vecs 的 `into_iter()` 产生 `i32`，我们想要引用它的一项，所以我们必须将 `&i32` 解构为 `i32`
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    // `into_iter()` for arrays yields `i32`
    println!(
        "Find 2 in array2: {:?}",
        array2.into_iter().find(|&x| x == 2)
    );
}
