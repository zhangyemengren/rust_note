/// 当然，trait也可以是通用的。在这里，我们定义了一个将 Drop trait重新实现为删除自身和输入的通用方法。

// Non-copyable types.
struct Empty;
struct Null;

// A trait generic over `T`.
trait DoubleDrop<T> {
    // 在调用者类型上定义一个方法，该方法接受一个附加的单个参数“T”并且不对其执行任何操作。
    fn double_drop(self, _: T);
}

// 为任何泛型参数“T”和调用者“U”实现“DoubleDrop<T>”。
impl<T, U> DoubleDrop<T> for U {
    // 此方法获取两个传递的参数的所有权，并释放两者。
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null  = Null;
    // Deallocate `empty` and `null`.
    empty.double_drop(null);

    // empty;
    //null;
    // ^ TODO: Try uncommenting these lines.
}