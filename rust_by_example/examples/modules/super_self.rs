/// 可以在路径中使用 super 和 self 关键字，以消除访问项目时的歧义并防止不必要的路径硬编码。

fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        // 让我们从此范围访问所有名为“function”的函数！
        print!("called `my::indirect_call()`, that\n> ");

        // “self”关键字指的是当前模块范围 - 在本例中为“my”。
        // 调用 self::function() 和直接调用 function() 都会给出相同的结果，因为它们引用相同的函数。
        self::function();
        function();

        // 我们还可以使用 self 来访问 my 中的另一个模块：
        // same as cool::function()
        self::cool::function();

        // “super”关键字指的是父作用域（在“my”模块之外）。
        super::function();

        // 这将绑定到 *crate* 范围内的 `cool::function`。
        // 在这种情况下，板条箱范围是最外面的范围。
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
