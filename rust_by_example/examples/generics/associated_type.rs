/// 对其容器类型通用的特征具有类型规范要求 - 该特征的用户必须指定其所有通用类型。
/// 在下面的示例中，Contains 特征允许使用泛型类型 A 和 B。
/// 然后为 Container 类型实现该特征，为 A 和 B 指定 i32，以便它可以与 fn Difference() 一起使用。
/// 因为 Contains 是通用的，所以我们被迫显式声明 fn Difference() 的所有通用类型。
/// 在实践中，我们想要一种方式来表达 A 和 B 由输入 C 确定。关联类型恰恰提供了这种功能。
/// “关联类型”的使用通过将内部类型本地移动到特征中作为输出类型来提高代码的整体可读性。特征定义的语法如下：
/// // “A”和“B”是通过“type”关键字在特征中定义的。
/// // （注意：此上下文中的“type”与用于别名时的“type”不同）。
/// trait Contains {
///     type A;
///     type B;
///
///     // 更新了语法以通用地引用这些新类型。
///     fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
/// }
/// // 使用时的区别(不再需要表达 A 或 B：)
/// // 不使用关联类型
/// fn difference<A, B, C>(container: &C) -> i32 where
///     C: Contains<A, B> { ... }
///
/// // 使用关联类型
/// fn difference<C: Contains>(container: &C) -> i32 { ... }

struct Container(i32, i32);

// 检查容器内是否存储有 2 个项目的特征。并检索第一个或最后一个值。
trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool; // 明确要求“A”和“B”。
    fn first(&self) -> i32; // 没有明确要求“A”或“B”。
    fn last(&self) -> i32; // 没有明确要求“A”或“B”。
}

impl Contains<i32, i32> for Container {
    // 如果存储的数字相等，则为 true。
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // 获取第一个数字。
    fn first(&self) -> i32 {
        self.0
    }

    // 获取最后一个数字。
    fn last(&self) -> i32 {
        self.1
    }
}

// “C”包含“A”和“B”。
// 鉴于此，必须表达“A”和 “B”又是一个麻烦。
fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
    container.last() - container.first()
}

// 此部分与上面的示例相同，但使用了关联类型。
trait AssociatedTypesContains {
    // 在这里定义方法可以使用的关联类型。
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl AssociatedTypesContains for Container {
    // 指定“A”和“B”是什么类型。
    // 如果“输入”类型是“Container(i32, i32)”，则“输出”类型被确定为“i32”和“i32”。
    type A = i32;
    type B = i32;

    // `&Self::A` 和 `&Self::B` 在这里也有效。
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn associated_types_difference<C: AssociatedTypesContains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);
    // Turbofish (::<>) 语法用于指定泛型参数的类型，但在 Rust 中并不适用于 trait 方法的选择。
    // Turbofish 语法用于在函数或方法调用中显式指定泛型类型参数，以帮助编译器进行类型推断或选择特定的实现。
    // 但它不能用于选择使用哪个具体的 trait 方法。
    // 在 Rust 中，如果一个类型实现了多个 trait，而这些 trait 都有相同的方法名，编译器无法自动确定你希望调用哪个具体的方法。
    // 因此，你需要使用完全限定语法（fully qualified syntax）来明确指定要调用的方法。
    println!(
        "Does container contain {} and {}: {}({})",
        &number_1,
        &number_2,
        Contains::contains(&container, &number_1, &number_2),
        AssociatedTypesContains::contains(&container, &number_1, &number_2)
    );
    // 完全限定语法 使用 as 转换
    println!(
        "First number: {}({})",
        <Container as Contains<i32, i32>>::first(&container),
        <Container as AssociatedTypesContains>::first(&container)
    );
    println!(
        "Last number: {}({})",
        <Container as Contains<i32, i32>>::last(&container),
        <Container as AssociatedTypesContains>::last(&container)
    );
    println!(
        "The difference is: {}({})",
        difference(&container),
        associated_types_difference(&container)
    );
}
