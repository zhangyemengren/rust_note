/// 某些函数与特定类型相关。它们有两种形式：关联函数和方法。
/// 关联函数是通常在类型上定义的函数，而方法是在类型的特定实例上调用的关联函数。

struct Point {
    x: f64,
    y: f64,
}

// Implementation block，所有与“Point”相关的函数和方法都在这里
impl Point {
    // 这是一个“关联函数”，因为该函数与特定类型，即 Point关联。
    // 关联函数不需要通过实例调用。
    // 这些函数通常像构造函数一样使用。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // 另一个关联函数，带有两个参数：
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // 这是一个方法
    // `&self` 是 `self: &Self` 的糖，其中 `Self` 是  调用者对象的类型。
    // 在这种情况下 `Self` = `Rectangle`
    fn area(&self) -> f64 {
        // `self` 通过点运算符提供对结构体字段的访问
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` 是一个 `f64` 的方法，返回绝对值
        // caller
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // 该方法要求调用者对象是可变的
    // `&mut self` 脱糖为 `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` 拥有资源：两个堆的分配
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // 该方法“消耗”调用者对象的资源(移动语义)
    // `self` 脱糖为 `self: Self`
    fn destroy(self) {
        // Destructure `self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
    }
}

fn main() {
    let rectangle = Rectangle {
        // 使用双冒号调用关联函数
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // 使用点运算符调用方法
    // 请注意，第一个参数 `&self` 是隐式传递的，即
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle perimeter: {}", Rectangle::perimeter(&rectangle));
    println!("Rectangle area: {}", rectangle.area());
    println!("Rectangle area: {}", Rectangle::area(&rectangle));

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };


    // 错误！ `rectangle` 是不可变的，但是这个方法需要一个可变的对象
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line

    // Okay! Mutable objects can call mutable methods
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // 错误！上一个 `destroy` 调用“消耗”了 `pair`
    //pair.destroy();
    // TODO ^ Try uncommenting this line
}