fn main() {
    println!("Hello, world!");

    //---------------------------------------------------------
    /*
    知识点：
        1、变量默认是不可变的（immutable）。
        2、变量名前添加 mut 来使其可变。
     */
    let mut x = 10;
    println!("x is {}", x);
    x = 20;
    println!("x is {}", x);

    //---------------------------------------------------------
    /*
    知识点：
        1、常量值总是不可变，使用 const 关键字声明，并且必须注明值的类型。
        2、常量可以在任何作用域中声明，包括全局作用域。
        3、常量的命名在单词之间用全大写加下划线。
     */
    const LOCAL_THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("LOCAL CONST is {LOCAL_THREE_HOURS_IN_SECONDS}");
    println!("GLOBAL CONST is {GLOBAL_THREE_HOURS_IN_SECONDS}");

    //---------------------------------------------------------
    /*
    知识点：
        1、定义相同名称的变量，第二个变量会将第一个变量隐藏，直到第二个变量离开作用域结束。
        2、再次使用 let 时，实际上是创建了一个新变量，可以改变值的类型，并复用这个名字。
     */

    let x = 10;
    let x = x + 5;
    {
        let x = x * 2;
        println!("x(second) is {x}");
    }
    println!("x(first) is {x}");

    let spaces = String::from("Hello world");
    let spaces = spaces.len();
    println!("spaces is {spaces}");

    //---------------------------------------------------------
    /*
    知识点：
        1、数据类型分为两种，标量和复合。每一个值都有一种数据类型，再有多种数据类型可能时，必须明确指定。
        标量：
            代表有整型，浮点型，布尔型和字符型。
        复合：
            代表有元组、数组。
     */

    let tup = (10, "Hello", true);
    println!("tup is ({}, {}, {})", tup.0, tup.1, tup.2);
    let (x, y, z) = tup;
    println!("tup is ({x}, {y}, {})", z);

    let array = [1, 2, 3, 4, 5];
    println!("array is {:?}", array);

    let array: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    println!("array is {:?}", array);

    //---------------------------------------------------------
    /*
    知识点：
        1、函数和变量名使用 snake case 规范风格。
        2、每个参数必须声明参数的类型。
        3、函数体由一系列的语句和一个可选的结尾表达式构成。
        4、Rust 是一门基于表达式的语言。
        5、语句时执行一些操作但不返回值的指令。表达式计算并产生一个值。
        6、标量是一个表达式，函数调用是一个表达式，宏调用是一个表达式，大括号创建的一个新的块作用域是一个表达式。
        7、函数的返回值等同于函数体最后一个表达式的值。使用 return 关键字和指定值，可以从函数中提前返回。
        8、单位类型（）,代表这个函数没有返回值。
     */

    another_function();
    another_function2(10);

    let x = {
        let y = 10;
        y * 2
    };
    println!("x is {}", x);

    let x = five();
    println!("x is {x}");

    //---------------------------------------------------------
    /*
    知识点：
        1、if 表达式中的值必须时 bool 值。
        2、因为 if 是一个表达式，我们可以在 let 语句的右侧使用它，返回值都必须时相同类型。
        3、循环分为 loop、while、for 三种

     */

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4, 3");
    }

    let condition = true;
    let number = if condition { 5 } else { 10 };
    println!("number is {}", number);

    let mut number = 0;
    loop {
        println!("number is {}", number);
        if number > 3 {
            break;
        }
        number += 1;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {result}");

    let mut count = 0;
    'continue_up: loop {
        println!("count is {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 0 {
                break;
            }
            if count == 2 {
                break 'continue_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("EOF");

    let a = [1, 2, 3, 4, 5];
    for e in a {
        println!("a is {e}");
    }
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("Another function. {}", x);
}

fn five() -> i32 {
    5
}

const GLOBAL_THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
