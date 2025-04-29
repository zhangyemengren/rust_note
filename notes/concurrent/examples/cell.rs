/// 内部可变性
use std::cell::{Cell, RefCell};

struct Counter{
    value: Cell<i32>
}

impl Counter{
    fn new() -> Counter{
        Counter{
            value: Cell::new(0)
        }
    }
    // 无需&mut self也能改变内部值
    fn increment(&self){
        self.value.set(self.value.get() + 1);
    }
    fn get(&self) -> i32{
        self.value.get()
    }
}
fn main(){
    let c = Counter::new();
    c.increment();
    c.increment();
    println!("{}", c.get());

    let s = Cell::new("hello".to_string());
    // 需实现Copy 才能调用get
    // s.get();
    // 可以生命为mut 来调用get_mut 返回底层数据的可变引用 但这样同时丧失了使用Cell的意义（内部可变性）
    let mut s = Cell::new("hello".to_string());
    println!("{}", s.get_mut());
    
    // RefCell
    let ref_cell = RefCell::new(5);
    // 符合借用规则 同时不能同时存在普通借用和可变借用
    {
        let borrowed = ref_cell.borrow();
        println!("{}", borrowed);
    }
    let borrowed = ref_cell.borrow();
    println!("{}", borrowed);
    drop(borrowed);
    let mut borrowed_mut = ref_cell.borrow_mut();
    *borrowed_mut += 1;
    // 实现了 Deref 自动解引用
    println!("{}", borrowed_mut.to_string());
}