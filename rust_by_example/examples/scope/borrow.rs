/// 大多数时候，我们希望访问数据而不取得数据的所有权。
/// 为了实现这一点，Rust 使用了借用机制。可以通过引用 (&T) 传递对象，而不是按值 (T) 传递对象。
/// 编译器静态地保证（通过其借用检查器）引用始终指向有效的对象。也就是说，当存在对对象的引用时，该对象不能被销毁。

// 该函数取得一个box的所有权并销毁它
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// 该函数借用了 i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    // 创建一个堆 i32 和一个栈 i32
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // 借用box里的东西。所有权未被夺取，
    // 这样内容就可以再次借用。
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // 取得一个对box里数据的引用
        let _ref_to_i32: &i32 = &boxed_i32;

        // Error!
        // 当稍后在范围内借用内部值时，无法销毁“boxed_i32”。
        // eat_box_i32(boxed_i32);
        // FIXME ^ Comment out this line

        // 内部值被销毁后尝试借用 `_ref_to_i32`
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` 超出范围并且不再被借用。
    }

    // `boxed_i32` 现在可以放弃所有权，移动到 `eat_box` 并被销毁
    eat_box_i32(boxed_i32);
}
