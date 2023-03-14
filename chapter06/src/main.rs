fn main() {
    println!("Hello, world!");

    /*
    1、枚举可以让你将一个值变成某一个集合中的一个的方法。
    2、每一个我们定义的枚举成员的名字，可以变成一个构建枚举的实例函数，用于存储值。
    3、枚举成员可以定义任何值。
    4、枚举类型也可以定义方法。
    5、Option 是标准库定义的一个枚举，覆盖一种场景：要么有值，要么没值。
    6、Option<T> 和 T 不是同一种类型。
     */

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    _ = four;
    _ = six;

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("192.168.95.118"),
    };
    println!("home is {:#?}, {:?}, {}", home, home.kind, home.address);

    #[derive(Debug)]
    enum IpAddr2 {
        V4(String),
        V6(String),
    }
    let home = IpAddr2::V4(String::from("192.168.95.118"));
    let loopback = IpAddr2::V6(String::from("::1"));
    println!("home is {:#?}, loopback is {:#?}", home, loopback);

    let quite = Message::Quit;
    let moved = Message::Move { x: 10, y: 2 };
    let write = Message::Write(String::from("hello world"));
    let change_color = Message::ChangeColor(10, 20, 30);

    println!(
        "message: {:#?}, {:#?}, {:#?}, {:#?}",
        quite, moved, write, change_color
    );
    write.call();
    moved.call();

    let x = Some(IpAddrKind::V4);
    let y: Option<i32> = None;
    _ = x;
    _ = y;

    /*
    1、match 是一种强大的控制流运算符，允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应的代码。
       模式可以由字面值、变量、通配符等其他内容构成。
    2、强大的 power 来源于模式的表现和编译器检查。
    3、match 关键字后跟一个表达式，这个表达式可以是任何类型。match 分支分为两部分：模式和一些代码。
    4、match 分支相关联的代码是一个表达式，而表达式的结果值将作为整个 match 表达式的返回值。
    5、match 分支必须覆盖所有的可能性。模式要进行穷举。
    6、_ 可以匹配任意模式，通配模式可以满足无穷的要求。
     */

    let penny = Coin::Penny;
    let cents = values_in_cents(penny);
    println!("cents is {}", cents);

    let _ = Coin::Nickel;
    let _ = Coin::Dime;
    let _ = Coin::Quarter;

    let quarter_alaska = Coin::Quarter(UsState::Alabama);
    let cents = values_in_cents(quarter_alaska);
    println!("cents is {}", cents);

    let five = Some(5);
    let six = plus_one(five);
    println!("six is {:#?}", six);

    let dice_roll = 10;
    match dice_roll {
        3 => println!("{}", 3),
        7 => println!("{}", 7),
        other => println!("{}", other),
    }

    /*
    1、if let 语法来处理只匹配一个模式的值而忽略其他模式的情况。
     */

    let config_max = Some(10);
    if let Some(max) = config_max{
        println!("the max is {}", max);
    }else{
        println!("None")
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn values_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state is {:#?}", state);
            25
        }
    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Move { x, y } => println!("x is {}, y is {}", x, y),
            other => {
                println!("other is {:#?}", other);
            }
        }
    }
}
