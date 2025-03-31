/// 创建一个在第一次遇到`None`后结束的迭代器。在迭代器返回`None`后，未来的调用可能会再次产生`Some(T)`，也可能不会。`fuse()`适配一个迭代器，确保在给出`None`后，它将永远只返回`None`。

struct Alternate {
    state: i32,
}
impl Iterator for Alternate {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.state;
        self.state += 1;
        if val % 2 == 0 { Some(val) } else { None }
    }
}
fn main() {
    let mut iter = Alternate { state: 0 };

    // 当iter.next()返回None后，下次依然会有值
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    // 使用fuse()后，iter.next()返回None后，总是返回None
    let mut iter = iter.fuse();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}
