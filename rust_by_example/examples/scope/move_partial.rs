/// 在单个变量的解构中，可以同时使用移动和引用模式绑定。
/// 这样做将导致变量的部分移动，这意味着变量的一部分将被移动，而其他部分将保留。
/// 在这种情况下，父变量以后不能作为整体使用，但是仅被引用（而不是移动）的部分仍然可以使用。
/// （在这个例子中，我们将age变量存储在堆上来说明部分移动：在下面的代码中删除ref会产生错误，因为person.age的所有权将被移动到变量age。
/// 如果Person.age被存储在堆栈上，不需要 ref，因为age的定义将从 person.age 复制数据而不移动它。）

fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` 已从 person 中移出，但引用了 `age`
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // 错误！借用部分移动的值：发生“person”部分移动 尝试在name前增加ref重试
    //println!("The person struct is {:?}", person);

    // 无法使用“person”，但可以使用“person.age”，因为它未移动
    println!("The person's age from person struct is {}", person.age);
}