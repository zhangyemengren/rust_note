/// 与函数类似，impl需要注意保持通用性。
/// struct S; // 具体类型 `S`
/// struct GenericVal<T>(T); // 泛型类型 `GenericVal`
/// // impl GenericVal 我们显式指定类型参数：
/// impl GenericVal<f32> {} // 指定 `f32`
/// impl GenericVal<S> {} // 指定上方已定义的 `S`
///
/// // `<T>` 必须位于类型之前才能保持通用
/// impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// impl of Val
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// impl of GenVal for a generic type `T`
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
}