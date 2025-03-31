/// copied 创建一个复制其所有元素的迭代器。当你有一个&T类型的迭代器，但需要一个T类型的迭代器时，这很有用。
///
/// cloned 创建一个迭代器，它clone其所有元素。当你有一个&T类型的迭代器，但需要一个T类型的迭代器时，这很有用。
/// 无法保证clone方法实际上会被调用或者被优化掉。所以代码不应该依赖于任何一种情况。
fn main() {
    // copied
    let a = [1, 2, 3];

    let v_copied: Vec<_> = a.iter().copied().collect();

    // copied is the same as .map(|&x| x)
    let v_map: Vec<_> = a.iter().map(|&x| x).collect();

    assert_eq!(v_copied, vec![1, 2, 3]);
    assert_eq!(v_map, vec![1, 2, 3]);

    // cloned
    let strings = vec!["hello".to_string(), "world".to_string()];

    // cloned 可以用于 String，因为 String 实现了 Clone 但没有实现 Copy
    let cloned_strings: Vec<String> = strings.iter().cloned().collect();

    // 下面这行如果使用 copied 会导致编译错误，因为 String 没有实现 Copy
    // let copied_strings: Vec<String> = strings.iter().copied().collect(); // 编译错误！

    assert_eq!(
        cloned_strings,
        vec!["hello".to_string(), "world".to_string()]
    );

    // 验证原始数据仍然可用
    println!("原始数据: {:?}", strings);
    println!("克隆数据: {:?}", cloned_strings);
}
