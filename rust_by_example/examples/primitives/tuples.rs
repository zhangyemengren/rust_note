/// 元组是不同类型值的集合。元组是使用括号()构造的，
/// 每个元组本身都是一个具有类型签名的值 (T1, T2, ...)，
/// 其中T1，T2是其成员的类型。函数可以使用元组返回多个值，因为元组可以保存任意数量的值。

// 元组可以用作函数参数和返回值。
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // let 可用于将元组的成员绑定到变量。
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // 具有多个不同类型的元组。
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // 可以使用元组索引从元组中提取值。
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // 元组可以作为元组的成员
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // 元组是可debug打印的
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // 但过长元组（超过12个元素）无法打印。
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    // 要创建一个元素的元组，需要使用逗号来分隔来自括号的元素 否则就是括号内的类型而非元组
    println!(
        "One element tuple: {:?} and the type is {}",
        (5u32,),
        std::any::type_name::<(u32,)>()
    );
    println!(
        "Just an integer: {:?} and the type is {}",
        (5u32),
        std::any::type_name::<(u32)>()
    );

    // 可以对元组进行解构以创建绑定。
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
    // 元组结构体
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}
