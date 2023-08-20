/// use 声明可用于将完整路径绑定到新名称，以便于访问。它经常这样使用：
/// use crate::deeply::nested::{
///     my_first_function,
///     my_second_function,
///     AndATraitType
/// };
///
/// fn main() {
///     my_first_function();
/// }
/// 您可以使用 as 关键字将导入绑定到不同的名称：

// Bind the `deeply::nested::function` path to `other_function`.
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    // 更轻松地访问“deeply::nested::function”
    other_function();

    println!("Entering block");
    {
        // 这相当于“use deeply::nested::function as function”。
        //  这个 `function()` 将遮蔽外部函数。
        use crate::deeply::nested::function;

        // `use` 绑定具有局部范围。在这种情况下，
        // `function()` 的隐藏仅在这个块中。
        function();

        println!("Leaving block");
    }

    function();
}