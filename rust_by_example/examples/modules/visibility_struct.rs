/// 结构体的字段具有额外的可见性。可见性默认为私有，并且可以使用 pub 修饰符覆盖。
/// 仅当从定义它的模块外部访问结构体时，这种可见性才重要，并且其目标是隐藏信息（封装）。


mod my {
    // 具有通用类型“T”公共字段的公共结构
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // 具有通用类型“T”私有字段的公共结构
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // 公共构造方法
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents,
            }
        }
    }
}

fn main() {
    // 具有公共字段的公共结构可以照常构建
    let open_box = my::OpenBox { contents: "public information" };

    // 并且他们的字段可以正常访问。
    println!("The open box contains: {}", open_box.contents);

    // 无法使用字段名称构造具有私有字段的公共结构。
    // Error! `ClosedBox` has private fields
    // let closed_box = my::ClosedBox { contents: "classified information" };
    // TODO ^ Try uncommenting this line

    // 但是，可以使用公共构造函数创建具有私有字段的结构
    let _closed_box = my::ClosedBox::new("classified information");

    // 但是无法访问公共结构的私有字段。
    // Error! The `contents` field is private
    // println!("The closed box contains: {}", _closed_box.contents);
    // TODO ^ Try uncommenting this line
}