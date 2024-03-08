/// Rust 提供了一个强大的模块系统，可用于按逻辑单元（模块）分层分割代码，并管理它们之间的可见性（公共/私有）。
/// 模块是项目的集合：函数、结构、特征、impl 块，甚至其他模块。
///
/// 默认情况下，模块中的项目具有私有可见性，
/// 但这可以使用 pub 修饰符覆盖。只能从模块范围之外访问模块的公共项。

// A module named `my_mod`
mod my_mod {
    // 模块中的项目默认为私有可见性。
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // 使用“pub”修饰符来覆盖默认可见性。
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // 项目可以访问同一模块中的其他项目，
    // 即使是私有的。
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // 模块也可以嵌套
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // 使用“pub(in path)”语法声明的函数仅在给定路径中可见。
        // `path` 必须是父模块或祖先模块
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // 使用 pub(self) 语法声明的函数仅在当前模块中可见，这与将它们保留为私有相同
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // 使用 pub(super) 语法声明的函数仅在父模块中可见
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) 使函数仅在当前 crate 内可见
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    // 嵌套模块遵循相同的可见性规则
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        // 私有父项目仍然会限制子项的可见性，即使它在更大的范围内声明为可见。
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // 模块允许消除具有相同名称的项目之间的歧义。
    function();
    my_mod::function();

    // 公共项目，包括嵌套模块内的项目，可以
    // 从父模块外部访问。
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) 项目可以从同一个 crate 中的任何地方调用
    my_mod::public_function_in_crate();

    // pub(in path) 项目只能从指定的模块内调用
    // Error! function `public_function_in_my_mod` is private
    //my_mod::nested::public_function_in_my_mod();
    // TODO ^ Try uncommenting this line

    // 模块的私有项不能直接访问
    // 即使嵌套在公共模块中：

    // Error! `private_function` is private
    //my_mod::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_function` is private
    //my_mod::nested::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::restricted_function();
    // TODO ^ Try uncommenting this line
}
