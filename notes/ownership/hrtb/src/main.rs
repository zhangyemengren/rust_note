fn main() {
    foo(|i| i);
}

fn foo<T: for<'a> Fn(&'a i32) -> &'a i32>(f: T) {
    for i in 0..5 {
        println!("{}", f(&i));
    }
}