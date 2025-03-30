/// flatten创建一个展平嵌套结构的迭代器。当你有一个迭代器的迭代器，或者一个可以转换为迭代器的事物的迭代器，并且你想要去除一层间接性时，这很有用。
/// 
/// flat_map 创建一个迭代器，其工作方式类似于map，但会展平嵌套结构。
fn main() {
    let a = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let b = a.iter().flatten().collect::<Vec<_>>();
    println!("{:?}", b);

    // flat_map对比flattern + map
    let words = ["alpha", "beta", "gamma"];
    let letters = words.iter().flat_map(|word| word.chars()).collect::<Vec<_>>();
    println!("{:?}", letters);

    let letters_flattern = words.iter().map(|word| word.chars()).flatten().collect::<Vec<_>>();
    println!("{:?}", letters_flattern);
}