#![allow(dead_code)]
/// 使用const 实现默认字段

// 初步实现 使用Default Trait,可以直接调用default生成默认值
#[derive(Debug, Default)]
struct Foo {
    alpha: &'static str,
    beta: bool,
    gamma: i32,
}
// 使用Default具有局限性 如想要自定义不同类型的默认值时
// 此时可以使用构造函数
impl Foo {
    fn new(alpha: &'static str, beta: bool) -> Self {
        Self {
            alpha,
            beta,
            gamma: 10,
        }
    }
    fn new2(alpha: &'static str, gamma: i32) -> Self {
        Self {
            alpha,
            beta: true,
            gamma,
        }
    }
}
// 构造函数的问题在于，调用者提供的每种字段组合都需要一个构造函数。为了解决这个问题，可以使用构建器
// 此版无法在编译时 保证必填参数（alpha beta） 必须设置
#[derive(Default)]
struct FooBuilder {
    pub alpha: Option<&'static str>,
    pub beta: Option<bool>,
    pub gamma: Option<i32>,
}

impl FooBuilder {
    fn new() -> Self {
        FooBuilder::default()
    }
    fn set_alpha(mut self, alpha: &'static str) -> Self {
        self.alpha = Some(alpha);
        self
    }
    fn set_beta(mut self, beta: bool) -> Self {
        self.beta = Some(beta);
        self
    }
    fn set_gamma(mut self, gamma: i32) -> Self {
        self.gamma = Some(gamma);
        self
    }

    fn build(self) -> Foo {
        Foo {
            alpha: self.alpha.unwrap(),
            beta: self.beta.unwrap(),
            gamma: self.gamma.unwrap_or(0),
        }
    }
}

// 使用const上下文保证编译时校验所有必填参数
#[derive(Default)]
pub struct FooBuilder2<const A: bool, const B: bool, const G: bool> {
    alpha: Option<&'static str>,
    beta: Option<bool>,
    gamma: Option<i32>,
}
impl FooBuilder2<false, false, false> {
    fn new() -> FooBuilder2<false, false, false> {
        FooBuilder2::default()
    }
}

// const 限制 调用一次后 无法在调用(范型参数 false -> true)
impl<const B: bool, const G: bool> FooBuilder2<false, B, G> {
    fn set_alpha(mut self, alpha: &'static str) -> FooBuilder2<true, B, G> {
        self.alpha = Some(alpha);
        unsafe { std::mem::transmute(self) } // 类型无改变 减少开销
    }
}
impl<const A: bool, const G: bool> FooBuilder2<A, false, G> {
    fn set_beta(mut self, beta: bool) -> FooBuilder2<A, true, G> {
        self.beta = Some(beta);
        unsafe { std::mem::transmute(self) }
    }
}
impl<const A: bool, const B: bool> FooBuilder2<A, B, false> {
    fn set_gamma(mut self, gamma: i32) -> FooBuilder2<A, B, true> {
        self.gamma = Some(gamma);
        unsafe { std::mem::transmute(self) }
    }
}
// 只能在alpha beta设置后才能调用（const 参数 为true时）
impl<const G: bool> FooBuilder2<true, true, G> {
    fn build(self) -> Foo { 
        Foo {
            alpha: self.alpha.unwrap(),
            beta: self.beta.unwrap(),
            gamma: self.gamma.unwrap_or(10), // This is an optional field with a default.
        }
    }
}

fn main() {
    let foo = Foo::default();
    let foo1 = Foo::new("foo", false);
    println!("使用Default Trait{:?}", foo);
    println!("使用构造函数{:?}", foo1);

    let foo2 = FooBuilder::new()
        .set_alpha("")
        .set_alpha("x")
        .set_beta(false)// 注释掉此行 运行时会报错 无法在编译时确认必须调用
        .set_gamma(20)
        .build();
    println!("构造器模式{:?}", foo2);

    let foo3 = FooBuilder2::new()
        .set_alpha("")
        // .set_alpha("x") 受const 范型限制 无法多次调用
        .set_beta(false)// 注释掉此行 编译时会报错
        .set_gamma(20)// 可以注释 调用build时 gamma无限制
        .build();
    println!("const改写的构造器模式{:?}", foo3);
}
