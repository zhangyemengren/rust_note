#![allow(dead_code)]

/// 可以使用 struct关键字创建三种类型的结构（“structs”）：
/// 元组结构体，基本上被命名为元组。
/// 经典的C 结构体
/// 单元结构体是无字段的，对于泛型很有用。

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 单元结构体
struct Unit;

// 元组结构体
struct Pair(i32, f32);

// 有两个字段的结构体
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段重用
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // 使用字段初始化 简写创建结构体
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    //实例化 a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 通过使用 struct update 语法来使用我们另一个的字段来创建一个新点
    // base struct 必须在最后的位置
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // 解构(需要声明具体结构体)
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // 结构体实例化也是一个表达式
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        // 当字段与值相同时 可省略冒号：和值
        bottom_right,
    };

    // 声明一个单元结构体实例
    let _unit = Unit;

    // 声明一个元组结构体实例
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体(顺序)
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
