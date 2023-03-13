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
