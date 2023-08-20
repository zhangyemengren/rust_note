/// 模块可以映射到文件/目录层次结构。让我们分解一下文件中的可见性示例：
/// tree .
/// .
/// ├── my
/// │   ├── inaccessible.rs
/// │   └── nested.rs
/// ├── my.rs
/// └── use.rs
/// 等价于
/// ├── my
/// │   ├── inaccessible.rs
/// │   └── nested.rs
/// │   └── mod.rs
/// └── use.rs
/// use.rs 使用my模块时需要声明模块mod my;
/// my.rs或my/mode.rs中 使用inaccessible和nested子模块需要声明模块
/// mod nested;
/// mod inaccessible;

fn main() {
    println!("Hello, world!");
}