/// 相同的规则集可以应用于函数：当前面带有 <T> 时，类型 T 变为泛型。
/// 使用泛型函数有时需要显式指定类型参数。如果在返回类型为泛型的情况下调用函数，或者编译器没有足够的信息来推断必要的类型参数，则可能会出现这种情况。
/// 具有显式指定类型参数的函数调用如下所示：fun::<A, B, ...>()。

struct A; // 具体类型 `A`.
struct S(A); // 具体类型 `S`.
struct SGen<T>(T); // 泛型类型 `SGen`.

// 以下函数都获得传递给它们的变量的所有权，并立即超出范围，释放该变量。

//  定义一个函数“reg_fn”，它接受“S”类型的参数“_s”。它没有`<T>`，所以这不是一个通用函数。
fn reg_fn(_s: S) {}

// 定义一个函数“gen_spec_t”，它接受类型为“SGen<T>”的参数“_s”。
// 它已被显式赋予类型参数“A”，但由于“A”尚未指定为“gen_spec_t”的泛型类型参数，因此它不是泛型的。
fn gen_spec_t(_s: SGen<A>) {}

// 定义一个函数“gen_spec_i32”，它采用“SGen<i32>”类型的参数“_s”。
// 它已被显式赋予类型参数“i32”，这是一种特定类型。
// 因为“i32”不是泛型类型，所以该函数也不是泛型。
fn gen_spec_i32(_s: SGen<i32>) {}

// 定义一个函数“generic”，它接受类型为“SGen<T>”的参数“_s”。
// 因为 `SGen<T>` 前面有 `<T>`，所以该函数是泛型的。
fn generic<T>(_s: SGen<T>) {}

fn main() {
    // 使用非泛型函数
    reg_fn(S(A)); // Concrete type.
    gen_spec_t(SGen(A)); // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(6)); // Implicitly specified type parameter `i32`.

    // 显式指定类型参数 `char` to `generic()`.
    generic::<char>(SGen('a'));

    // 隐式指定类型参数 `char` to `generic()`.
    generic(SGen('c'));
}
