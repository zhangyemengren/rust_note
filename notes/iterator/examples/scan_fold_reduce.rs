/// “fold ()” 对每个元素进行操作并折叠到累加器中，返回最终结果。
/// “fold ()” 接受两个参数：初始值和一个带有两个参数的闭包，即 “累加器” 和一个元素。
/// 闭包返回累加器在下一次迭代中应具有的值。初始值是累加器在第一次调用时的值。在对迭代器的每个元素应用此闭包后，“fold ()” 返回累加器。此操作有时称为 “reduce（归约）” 或 “inject（注入）”。
/// 当你有一系列的东西并且想从中产生一个单一的值时，折叠是很有用的。
///
///
/// scan就像fold一样，持有内部状态，但与fold不同的是，它会生成一个新的迭代器。
/// scan()接受两个参数：一个初始值用于初始化内部状态，以及一个闭包，这个闭包有两个参数，第一个是对内部状态的可变引用，第二个是迭代器元素。闭包可以对内部状态进行赋值，以便在迭代之间共享状态。
/// 在迭代过程中，闭包将应用于迭代器的每个元素，并且闭包的返回值（一个Option）将由next方法返回。因此，闭包可以返回Some(value)以产生值（value
///
/// reduce 通过反复应用归约操作将元素归约为单个元素。如果迭代器为空，则返回None；
/// 否则，返回归约的结果。归约函数是一个带有两个参数的闭包：一个“累加器”和一个元素。
/// 对于至少有一个元素的迭代器，这与fold()相同，将迭代器的第一个元素作为初始累加器值，并将每个后续元素折叠到其中。
fn main() {
    // scan示例 - 返回一个迭代器
    let a = [1, 2, 3, 4, 5];
    let iter = a.iter().scan(0, |state, x| {
        println!("scan state: {}", state);
        println!("scan x: {}", x);
        *state += x;
        Some(*state)
    });
    println!("scan结果: {:?}", iter.collect::<Vec<_>>());

    // fold示例 - 直接返回累积结果
    let a = [1, 2, 3, 4, 5];
    let result = a.iter().fold(0, |acc, &x| {
        println!("fold acc: {}", acc);
        println!("fold x: {}", x);
        acc + x
    });
    println!("fold结果: {}", result);

    // 另一个fold示例 - 计算数组元素的乘积
    let product = a.iter().fold(1, |acc, &x| acc * x);
    println!("元素乘积: {}", product);

    // reduce示例
    // 与fold 0类似，但返回一个Option（如果迭代器为空，则返回None）
    // 此处直接使用range没有调用iter 所以没有解构x
    let sum = (0..5).reduce(|acc, x| acc + x).unwrap();
    println!("reduce结果: {}", sum);
}
