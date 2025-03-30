/// 方法调用的自动解引用 隐式deref https://doc.rust-lang.org/std/ops/trait.Deref.html#tymethod.deref
fn main() {
    struct Point { x: i32, y: i32 }
    impl Point {
        fn distance(&self) -> f64 {
            ((self.x * self.x + self.y * self.y) as f64).sqrt()
        }
    }

    let point = Point { x: 3, y: 4 };
    let point_ref = &point;
    let point_ref_ref = &&point;

    println!("{}", point.distance());
    println!("{}", point_ref.distance());
    println!("{}", point_ref_ref.distance());
    
}