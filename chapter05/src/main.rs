use std::println;

fn main() {
    println!("Hello, world!");
    /*
    1、结构体不同于元组的地方在于需要命名各部分数据以便能清楚的表明其值的意义。
    2、一旦定义了结构体后，通过为每个字段指定具体值来创建这个结构体的实例。
    3、整个实例必须是可变的;Rust 不允许只将某个字段标记为可变。
    4、如果属性字段名称和参数相同，可以使用简短写法。
    5、使用结构体更新语法，可以只更新变化的字段，其他的字段服用外来变量，但要注意所有权和末尾没有 , 的写法。
    6、元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。元组名称不同，即使组成相同，也不相等。
    7、没有任何字段的类单元结构体，通常用来实现 trait 行为。
     */

    let user1 = User {
        active: true,
        username: String::from("shepard"),
        email: String::from("eviltomorrow@163.com"),
        sign_in_count: 1,
    };
    println!("user is {:#?}", user1);
    println!(
        "user is {}, {}, {}, {}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );

    let u3 = build_user(
        String::from("shepard"),
        String::from("eviltomorrow@163.com"),
    );
    println!("{:#?}", u3);

    let mut user2 = User {
        username: String::from("captain"),
        ..user1
    };
    user2.username = String::from("liarsa");
    println!("user2 is {:?}", user2);

    println!("user1 is {}", user1.active);

    let black = Color(0, 0, 0);
    println!("black is {:#?}", black);
    println!("color.0 is {}", black.0);
    let (x, y, z) = black;
    println!("x, y, z is {}, {}, {}", x, y, z);

    /*
    1、println! 宏能处理很多类型，这些基本常用的类型，默认都实现了 Display。
    2. dbg! 接收引用返回表达式的所有权。
     */

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!(
        "rect1 is {:#?} \r\n, width: {}, height: {}",
        rect1, rect1.width, rect1.height
    );

    /*
    1、方法总是被定义在结构体的上下文中，并且他们的第一个参数总是 self，代表调用该方法的结构体实例。
    2、&self 实际上是 self: &self 的缩写。
    3、在 impl 中定义的函数被称作关联函数。
    4、每个方法都允许有多个 impl 块，但是每个方法有其自己的 impl 块。
     */

    let rect1 = Rectangle {
        width: 20,
        height: 10,
    };
    let area = rect1.area();
    println!("area is {}", area);

    let rect2 = Rectangle {
        width: 30,
        height: 20,
    };
    let flag = rect1.can_hold(&rect2);
    println!("flag is {}", flag);

    let rect3 = Rectangle::new(30, 20);
    println!("rect3 is {:#?}", rect3);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}

#[derive(Debug)]
struct Color(i32, i32, i32);
