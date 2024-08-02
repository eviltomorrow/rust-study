fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?}, {:#?}", four, six);

    route(four);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    home.println();
    println!("{:#?}, {:?}", home, loopback);

    let home = IpAddrMode::V4(String::from("127.0.0.1"));
    let loopback = IpAddrMode::V6(String::from("::1"));
    println!("{:?}, {:?}", home, loopback);
    home.println();

    let home = IpAddrMode2::V4(192, 168, 0, 122);
    let loopback = IpAddrMode2::V6(String::from("::1"));
    println!("{:?}, {:?}", home, loopback);

    match home {
        IpAddrMode2::V4(a, b, c, d) => {
            println!("{a},{b:?},{c},{d}");
        }
        IpAddrMode2::V6(ip) => {
            println!("{}", ip);
        }
    }

    let write = Message::Write(String::from("Hello world"));
    write.call();

    let data = come_back(true, "Hello world");
    match data {
        Some(s) => println!("{}", s),
        None => println!("no"),
    }

    let coin1 = Coin::Penny;
    let c = value_in_cent(coin1);
    println!("{}", c);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:#?}", six, none);

    try_to_dice_roll(10);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let coin2 = Coin::Quarter(UsState::Alabama);

    let mut count = 0;
    if let Coin::Quarter(state) = coin2 {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
    println!("{count}");
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrMode {
    V4(String),
    V6(String),
}

impl IpAddrMode {
    fn println(&self) {
        match self {
            IpAddrMode::V4(data) => {
                println!("{}", data);
            }
            IpAddrMode::V6(data) => {
                println!("{data}");
            }
        }
    }
}

#[derive(Debug)]
enum IpAddrMode2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn println(&self) {
        println!("{:?}, {:?}", self.kind, self.address);
    }
}

fn route(ip_kind: IpAddrKind) {
    println!("{:#?}", ip_kind);
}

#[derive(Debug)]
enum UsState {
    Alabama,
}

enum Coin {
    Penny,
    Quarter(UsState),
}

fn value_in_cent(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        // Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{state:#?}",);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match &self {
            Self::Quit => println!("quite"),
            Self::Move { x, y } => println!("move {}, {}", x, y),
            Self::Write(text) => println!("write {}", text),
            Self::ChangeColor(_, _, _) => println!("change color"),
        }
    }
}

fn come_back(ok: bool, s: &str) -> Option<String> {
    let s = String::from(s);
    if ok {
        Some(s)
    } else {
        None
    }
}

fn try_to_dice_roll(dice_roll: i32) {
    match dice_roll {
        3 => println!("add"),
        8 => println!("sub"),
        _ => (),
    }
}
