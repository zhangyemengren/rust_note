fn main() {
    // 解构元组
    let point = (3, 4);
    let (x, y) = point;
    println!("x: {}, y: {}", x, y);

    // 解构数组 没解构部分需要省略语法
    let arr = [1, 2, 3, 4, 5];
    let [first, second, ..] = arr;
    println!("first: {}, second: {}", first, second);

    // 解构结构体 没解构部分需要省略语法
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    let point = Point { x: 3, y: 4, z: 5 };
    let Point { x, y, .. } = point;
    println!("x: {}, y: {} z: {}", x, y, point.z);

    // 解构嵌套
    let pairs = vec![(1, 2), (3, 4)];
    for &(a, b) in pairs.iter() {
        // 解构&(i32, i32)为(i32, i32)，然后再解构为i32
        println!("{}, {}", a, b);
    }
    // 解构函数参数
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("坐标：({}, {})", x, y);
    }
    let point = (3, 4);
    print_coordinates(&point);

    // 双重解构
    let a = [0, 1, 2];
    let _iter = a.iter().filter(|&&x| x > 1); // 使用&&x模式直接解构出i32值
}
