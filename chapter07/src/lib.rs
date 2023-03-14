pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

/*
1、模块可以让我们将一个 crate 中的代码进行分组，以提高可读性与重用性。
2、模块中可以定义子模块，结构体，枚举，常量， 特性或者函数。
3、找到一个项的位置，我们使用路径。
    - 绝对路径：以 crate 开头的全路径;外部：crate 名开头的绝对路径，当前：以字面值 crate 开头。
    - 相对路径：从当前模块开始，以 self、super 或当前模块的标志符开头。
4、::分割
5、对于使用绝对路径还是相对路径，主要看相互引用的代码是否共同移动。
6、子模块对于父模块是私有的，父模块的上下文对子模块是感知的，兄弟模块间不需要 pub 关键字。
7、结构体的定义前使用 pub，这个结构体会变成公有的，但这个结构体的字段仍然是私有的。所以，我们需要一个静态函数来创建实例。
8、枚举的定义前使用 pub，枚举的所有成员都是公有的。
9、use 可以创建一个短路径，然后就可以在作用域中的任何地方使用这个短名称。最后一个名称通常为父层级。
10、use 只能创建 use 所在的特定作用域内的短路径。
11、特例：相同名称的项带入到作用域后，从属于不同的父层级，需要注明父模块。或者，使用 as 关键字做一个别名，进行区分。
12、重导出：pub 和 use 联合使用，将一个名称导入当前作用域，还允许别人把它导入他们自己的作用域。
13、嵌套路径和 self 写法，glob 运算符 *。
14、路径拆分为多个文件。参照文件目录结构，和 mod 名称相同。
 */

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        self::cook_order();
        super::deliver_order();
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,

        #[warn(dead_code)]
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("apple"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    back_of_house::fix_incorrect_order();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast pelase", meal.toast);

    let soup = back_of_house::Appetizer::Soup;
    _ = soup;
    let _ = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
}

use rand::Rng;
pub fn random_num() {
    let num = rand::thread_rng().gen_range(1..=200);
    println!("{num}");
}

// use std::{self,Write};
// use std::{cmp::Ordering, io};
// use std::fmt;
// use std::io;
// use std::io::Result as IoResult;

// pub fn f1() -> fmt::Result{

// }

// pub fn f2() -> io::Result<()>{

// }

// pub fn f3() -> IoResult<()>{

// }

/*
1、一个包可以包含多个二进制 crate 和一个可选的 crate 库。至少包含一个 crate。
2、同一作用域内不能拥有两个相同名称的项。
3、模块系统：
    - 包：Cargo 的一个功能，它允许你构建、测试和分享 crate。
    - Crates： 一个模块的树形结构，它形成了库或二进制项目。
    - 模块（modules）和 use：允许你控制作用域和路径的私有性。
    - 路径（path）：一个命名如结果头、函数或模块等项的方式。
4、crate 是 Rust 在编译时最小的代码单位，可以包含模块。形式：二进制项和库。
5、creat root 是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 的根模块。
6、包是提供一系列功能的一个或者多个 crate。一个包会包含一个 Cargo.toml 文件。
7、cargo 约定：
    - src/main.rs，就是一个与包同名的二进制 crate 的 crate 根。
    - src/lib.rs，则是带有与其同名的库 crate 的 crate 根。
 */
