/// 闭包是可以捕获封闭环境的函数。例如，捕获x变量的闭包：
/// |val| val + x
/// 闭包的语法和功能使它们非常方便即时使用。调用闭包与调用函数完全相同。
/// 但是，输入和返回类型都可以推断，并且必须指定输入变量名称。
/// 闭包的其他特征包括：
/// 1 使用||而不是()围绕输入变量。
/// 2 单个表达式主体分隔是可选的 {}（否则强制）e.g. |val| val + x。 || {println!("{x}"); x + 1}。
/// 3 捕获外部环境变量的能力。

fn main() {
    let outer_var = 42;

    // 常规函数不能引用封闭环境中的变量
    // fn function(i: i32) -> i32 { i + outer_var }

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    // 闭包是匿名的，这里我们将它们绑定到引用
    // 注解与函数注解相同但是是可选的，就像包装主体的“{}”一样。这些无名函数被分配给适当命名的变量。
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i| i + outer_var;

    // Call the closures.
    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));
    // 一旦推断出闭包的类型，就无法通过另一种类型再次推断出来。
    // println!("cannot reuse closure_inferred with another type: {}", closure_inferred(42i64));
    // TODO: uncomment the line above and see the compiler error.

    // 不带参数的闭包返回“i32”。
    // 返回类型是推断的。
    let one = || 1;
    println!("closure returning one: {}", one());
}
