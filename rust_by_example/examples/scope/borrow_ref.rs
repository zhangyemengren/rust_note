/// 通过 let 绑定进行模式匹配或解构时，可以使用 ref 关键字来引用结构体/元组的字段。
/// 下面的示例显示了一些有用的实例：

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let c = 'Q';

    // 赋值左侧的“ref”借用 相当于右侧的“&”借用。
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // 解构结构时，“ref”也有效。
    let _copy_of_x = {
        // `ref_to_x` 是对 `point` 的 `x` 字段的引用。
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;

        // 返回“point”的“x”字段的副本。
        *ref_to_x
    };

    // `point` 的可变副本
    let mut mutable_point = point;

    {
        // ref` 可以与 `mut` 配对以获取可变引用。
        let Point {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point;

        // 通过可变引用改变“mutable_point”的“y”字段。
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!(
        "mutable_point is ({}, {})",
        mutable_point.x, mutable_point.y
    );

    // 包含指针的可变元组
    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        // 解构“mutable_tuple”以更改“last”的值。
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }

    println!("tuple is {:?}", mutable_tuple);
}
