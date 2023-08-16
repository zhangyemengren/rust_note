/// 原始类型可以通过强制转换 as 相互转换。
/// Rust 通过使用特征 trait 来解决自定义类型（即结构和枚举）之间的转换。
/// 通用转换将使用 From 和 Into 特征。 然而，对于更常见的情况，有更具体的方法，特别是在与字符串之间转换时。

// From 特征允许类型定义如何从另一种类型创建自身，
// 从而提供了一种非常简单的机制来在多种类型之间进行转换。标准库中有许多此特征的实现，用于原始类型和常见类型的转换。

// Into 只是From特征的倒数。也就是说，如果您已经为您的类型实现了From特征，Into则会在必要时调用它。
// 使用Into特征通常需要指定要转换的类型，因为编译器大多数时候无法确定这一点。然而，考虑到我们免费获得该功能，这是一个小小的权衡。

// 与 From 和 Into 类似，TryFrom 和 TryInto 是用于在类型之间进行转换的通用特征。
// 与 From/Into 不同，TryFrom/TryInto 特征用于易出错的转换，因此返回Results。
use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// 将任何类型转换为 String 就像为该类型实现 ToString 特征一样简单。
// 您应该实现 fmt::Display 特征，而不是直接这样做，
// 它会自动提供 ToString 并且还允许打印类型，如 print! 部分中讨论的那样。
struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);

    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
    // 转换为字符串
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
    // 将字符串转换为最常见的类型之一是数字。 惯用的方法是使用parse函数，
    // 并使用类型推断或使用“turbofish”语法指定要解析的类型。 以下示例显示了两种替代方案。
    // 只要为该类型实现了 FromStr 特征，这就会将字符串转换为指定的类型。
    // 标准库中的许多类型都实现了这一点。 要在用户定义的类型上获得此功能，只需为该类型实现 FromStr 特征即可。
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}