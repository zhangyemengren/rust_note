/// 数据可以被不可变地借用任意次数，但是在有不可变地借用的同时，原始数据不能被可变地借用。
/// 另一方面，一次只允许一次可变借用。只有最后一次使用可变引用后，才能再次借用原始数据。

struct Point { x: i32, y: i32, z: i32 }

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };
    // 多次不可变借用
    let borrowed_point = &point;
    let another_borrow = &point;

    // 可以通过借用和原始所有者访问数据
    println!("Point has coordinates: ({}, {}, {})",
             borrowed_point.x, another_borrow.y, point.z);

    // Error! 无法将“point”作为可变借用，因为它当前已作为不可变借用。
    // let mutable_borrow = &mut point;
    // TODO ^ Try uncommenting this line

    // 此处再次使用借用的值
    println!("Point has coordinates: ({}, {}, {})",
             borrowed_point.x, another_borrow.y, point.z);

    // 不可变引用不再用于代码的其余部分，因此可以使用可变引用重新借用。
    let mutable_borrow = &mut point;

    // 通过可变引用更改数据
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // Error! 无法将“point”作为不可变借用，因为它当前被借用为可变。
    // let y = &point.y;
    // TODO ^ Try uncommenting this line

    // Error! 无法打印，因为 `println!` 采用不可变引用。
    // println!("Point Z coordinate is {}", point.z);
    // TODO ^ Try uncommenting this line

    // 好的！可变引用可以作为不可变引用传递给 `println!`
    println!("Point has coordinates: ({}, {}, {})",
             mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    // 可变引用不再用于代码的其余部分，因此可以重新借用
    let new_borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
             new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);
}