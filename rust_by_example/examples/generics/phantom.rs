/// 虚类型参数是一种在运行时不显示的参数，但在编译时（且仅）静态检查。
/// 数据类型可以使用额外的泛型类型参数来充当标记或在编译时执行类型检查。这些额外参数不保存存储值，并且没有运行时行为。
/// 在下面的示例中，我们将 std::marker::PhantomData 与虚类型参数概念结合起来，创建包含不同数据类型的元组。
use std::marker::PhantomData;

// 一个虚元组结构，它是带有隐藏参数“B”的“A”的通用结构。
#[derive(PartialEq)] // 允许对该类型进行相等性测试。
struct PhantomTuple<A, B>(A, PhantomData<B>);

// 一个虚类型结构体，它是' A '的泛型，具有隐藏参数' B '。
#[derive(PartialEq)] // 允许对该类型进行相等性测试。
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

// 注意:为泛型类型' A '分配存储，但不为' B '分配存储。
// 因此，' B '不能用于计算。

fn main() {
    // 这里，' f32 '和' f64 '是隐藏参数。
    // PhantomTuple类型指定为' <char, f32> '。
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple3: PhantomTuple<char, f32> = PhantomTuple('W', PhantomData);
    let _tuple4: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // PhantomTuple类型指定为' <char, f64> '。
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // 类型指定为' <char, f32> '。
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // Type specified as `<char, f64>`.
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    println!("_tuple1 == _tuple3 yields: {}", _tuple1 == _tuple3);
    println!("_tuple1 == _tuple4 yields: {}", _tuple1 == _tuple4);
    // 编译时错误!类型不匹配，因此无法进行比较:
    // println!("_tuple1 == _tuple2 yields: {}",
    //           _tuple1 == _tuple2);

    // 编译时错误!类型不匹配，因此无法进行比较:
    // println!("_struct1 == _struct2 yields: {}",
    //           _struct1 == _struct2);
}
