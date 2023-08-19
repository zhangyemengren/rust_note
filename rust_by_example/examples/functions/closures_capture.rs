/// 捕获
/// 闭包本质上是灵活的，并且将执行功能所需的操作，使闭包无需类型注解即可工作。
/// 这使得捕获能够灵活地适应用例，有时移动，有时借用。闭包可以捕获如下变量：
/// 引用：&T
/// 可变引用：&mut T
/// 值：T
/// 它们优先通过引用捕获变量，并且仅在需要时才降低。

fn main() {
    use std::mem;

    let color = String::from("green");

    // 打印“color”的闭包，它立即借用（“&”）“color”并将借用和闭包存储在“print”变量中。
    // 它将保持借用状态，直到最后一次使用“print”。
    //
    // `println!` 仅需要不可变引用的参数，因此它不会施加任何更多限制。
    let print = || println!("`color`: {}", color);

    // 使用借用调用闭包。
    print();

    // `color` 可以再次被不可变地借用，因为闭包只保存对 `color` 的不可变引用。
    let _reborrow = &color;
    print();

    // 最终使用“print”后允许移动或重新借用
    let _color_moved = color;


    let mut count = 0;
    // 增加 `count` 的闭包可以采用 `&mut count` 或 `count`
    // 但 `&mut count` 的限制较少，因此需要这样做。立即借用“count”。
    //
    // `inc` 上需要一个 `mut`，因为里面存储了一个 `&mut`。
    // 因此，调用该闭包需要 `mut` 。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 使用可变借用调用闭包。
    inc();

    // 闭包仍然可变地借用“count”，因为它是稍后调用的。
    // 尝试重新借用将导致错误(已有mut借用)。
    // let _reborrow = &count;
    // ^ TODO: try uncommenting this line.
    inc();


    // 闭包不再需要借用“&mut count”。因此它是
    // 因此它可以无错误地重新借用
    let _count_reborrowed = &mut count;


    // 非Copy类型。
    let movable = Box::new(3);


    // `mem::drop` 需要 `T`，所以这必须按值获取。
    // copy类型将复制到闭包中，而不影响原始类型。非Copy类型必须移动，因此“可移动值”立即移动到闭包中。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` 消耗变量，因此只能调用一次。
    consume();
    // consume();
    // ^ TODO: Try uncommenting this line.
}