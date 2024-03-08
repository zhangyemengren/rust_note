/// 使用泛型时，类型参数通常必须使用特征作为边界来规定类型实现的功能。
/// 例如，下面的示例使用 Display 特征来打印，因此需要 T 被 Display 绑定；也就是说，T 必须实现 Display。
/// 定义一个函数“printer”，它采用泛型类型“T”，其必须实现特征“Display”。
/// fn printer<T: Display>(t: T) {
///     println!("{}", t);
/// }
/// 边界将泛型限制为符合边界的类型。那是：
/// struct S<T: Display>(T);
/// 错误！ `Vec<T>` 没有实现 `Display`。这实例化将会失败
/// let s = S(vec![1]);
/// 边界的另一个作用是允许泛型实例访问边界中指定的特征的方法。
// 实现打印标记`{:?}`的特征
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}
#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

// 通用“T”必须实现“Debug”。无论哪种类型，这都会正常工作。
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` 必须实现 `HasArea`。任何满足的类型绑定
// 可以访问“HasArea”的函数“area”。
fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    //print_debug(&_triangle);
    //println!("Area: {}", area(&_triangle));
    // ^ TODO: Try uncommenting these.
    // | Error: Does not implement either `Debug` or `HasArea`.
}
