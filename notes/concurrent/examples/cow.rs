/// Cow 写时克隆
use std::borrow::Cow;

fn main() {
    let s = "Hello World";
    let cow = Cow::Borrowed(s);
    // into_owned 借用会执行clone
    assert_eq!(cow.into_owned(), "Hello World");

    let s = "Hello World";
    let cow: Cow<'_, str> = Cow::Owned(s.to_string());
    // 此时move
    assert_eq!(cow.into_owned(), "Hello World");

    // to_mut 取得数据所有权形式的可修改引用。若数据当前为借用状态，则执行克隆操作
    let mut cow = Cow::Borrowed("Hello World");
    cow.to_mut().make_ascii_uppercase();
    assert_eq!(cow, Cow::<'_, str>::Owned("HELLO WORLD".to_string()));
    println!("{}", cow);
}
