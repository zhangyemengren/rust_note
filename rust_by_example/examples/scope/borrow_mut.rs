/// 可以使用 &mut T 可变地借用可变数据。这称为可变引用，并为借用者提供读/写访问权限。
/// 相比之下，&T 通过不可变引用借用数据，借用者可以读取数据但不能修改它：
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` 是对分配在只读内存中的字符串的引用
    author: &'static str,
    title: &'static str,
    year: u32,
}

// 这个函数需要获取book的引用
fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

// 此函数引用可变book并将“年份”更改为 2014 年
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    // 创建一个名为“immutabook”的不可变book
    let immutabook = Book {
        // string literals have type `&'static str`
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // 创建“immutabook”的可变副本并将其命名为“mutabook”(Book impl Copy trait)
    let mut mutabook = immutabook;

    // 不可变地借用一个不可变的对象
    borrow_book(&immutabook);

    // 不可变地借用可变对象
    borrow_book(&mutabook);

    // 可变地借用可变对象
    new_edition(&mut mutabook);

    // Error! 不能可变地借用不可变对象
    // new_edition(&mut immutabook);
    // FIXME ^ Comment out this line
}
