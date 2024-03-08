/// 边界也可以使用紧接在{ 开头 之前的 where 子句来表达，而不是在类型第一次提及时表达。
/// 此外，where 子句可以将边界应用于任意类型，而不仅仅是类型参数。
/// 在某些情况下，where 子句很有用：
/// 当单独指定泛型类型和界限时会更清晰：
/// impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}
///
/// // 用“where”子句表达边界
/// impl <A, D> MyTrait<A, D> for YourType where
///     A: TraitB + TraitC,
///     D: TraitE + TraitF {}
/// 使用 where 子句比使用普通语法更具表现力。
/// 下例中的 impl 不能在没有 where 子句的情况下直接表达：
use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// 因为我们必须将其表达为“T: Debug”或使用另一种间接方法，所以这需要一个“where”子句：
impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    // 我们希望“Option<T>: Debug”作为我们的边界，因为这就是要打印的内容。
    // 否则就会使用错误的界限。
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
