fn main() {
    println!("Hello, world!");
    let mut x = 20;
    x += 20;
    println!("x is {x}");

    say();
    say_something("shepard");
    println!("return {}", return_something());
    println!("{}", THREE_HOURS_SECONDS);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

    let a = 10.0;
    let b: f64 = 20.0;
    let sum = a + b;
    println!("sum is {}", sum);
    let sum = a * b;
    println!("sum is {}", sum);

    let c: f64 = 30.0;
    println!("sum is {}", a + c);

    let t: bool = false;
    if !t {
        println!("t is {}", t);
    }

    let mut c = 'a';
    println!("c is {c}");
    c = 'b';
    println!("c is {}", c);
    c = '你';
    println!("c is {}", c);

    let tup = (3, 2.0, 'c', true);
    println!("tup is {:#?}", tup);

    let a = [1, 2, 3];
    println!("{:?}", a);
    println!("{:?}", a);

    for e in a {
        println!("e is {:?}", e);
    }

    let mut count = 0;
    loop {
        count += 1;
        if count >= 10 {
            break;
        }
        println!("count-{}", count);
    }

    while count > 5 {
        count -= 1;
        println!("count-{count}");
    }

    let s1 = String::from("world");
    takes_ownership(s1);
    // println!("{s1}");

    let x = 5;
    make_copy(x);
    println!("{x}");

    let s = gives_ownership();
    let s = takes_and_give_back(s);
    println!("{s}");

    let (s, len) = calculate_length(s);
    println!("s is {s}, len is {len}");

    let l = calculate_length2(&s);
    println!("s is {s}, len is {l}");

    let loc = first_word(&s);
    println!("loc is {loc}");

    let hello = first_word2(&s);
    println!("hello is {hello}");

    let mut s = String::from("");
    s.push('a');
    println!("{s}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:#?}", &a[1..2]);

    let mut s = String::from("Hi");
    let a = &mut s;
    change_word(a);
    let b = &s;
    println!("{}", b);

    let user = User {
        active: true,
        username: String::from("shepard"),
        email: String::from("eviltomorrow@163.com"),
        sign_in_count: 20,
    };
    println!(
        "{:?}, {}, {}, {}, {}",
        user, user.active, user.username, user.email, user.sign_in_count
    );

    let username = String::from("Hello");
    let user = new_user(username, String::from("eviltomorrow@163.com"));
    println!(
        "{:?}, {}, {}, {}, {}",
        user, user.active, user.username, user.email, user.sign_in_count,
    );

    let user1 = User {
        username: String::from("u2"),
        ..user
    };
    println!(
        "{:?}, {}, {}, {}, {}",
        user1, user1.active, user1.username, user1.email, user1.sign_in_count,
    );

    let black = Color(0, 0, 0);
    let origin = Point(1, 2, 3);
    println!("{:?}, {:?}", black, origin);

    let ae = AlwaysEqual;
    println!("{:?}", ae);

    let rec = Rectangle {
        name: String::from("长方形"),
        width: 10,
        height: 20,
    };

    let sum_area = area(&rec);
    println!("name {}'s sum_area is {}", rec.name, sum_area);

    println!("name {}'s area {}", rec.name, rec.area());

    let rec1 = Rectangle {
        name: String::from("rec1"),
        width: 10,
        height: 20,
    };

    let rec2 = Rectangle {
        name: String::from("rec2"),
        width: 5,
        height: 10,
    };
    let flag = rec1.can_hold(&rec2);
    println!("flag is {}", flag);

    let rec = Rectangle::new();
    println!("{:?}", rec);

    let _quit = Message::Quit;
    let _move = Message::Move { x: 10, y: 20 };
    let _write = Message::Write(String::from("Hello world"));
    let _change_color = Message::ChangeColor(10, 10, 10);
    _move.call();

    if let Message::ChangeColor(x, y, z) = _change_color {
        println!("{}, {}, {}", x, y, z);
    }

    let tup = (
        1i32,
        2u32,
        true,
        "hello",
        String::from("hello"),
        Rectangle {
            width: 10,
            height: 10,
            name: String::new(),
        },
    );
    println!("{:#?}", tup);

    let a = 10;
    let b = 20;
    let sum = a + b;
    println!("{sum}",);
}

const THREE_HOURS_SECONDS: i64 = 32 * 60 * 60;

fn say() {
    println!("hello world");
}

fn say_something(world: &str) {
    println!("hello {}", world);
}

fn return_something() -> String {
    String::from("Hello world")
}

fn takes_ownership(s: String) {
    println!("{s}");
}

fn make_copy(x: i32) {
    println!("{x}");
}

fn gives_ownership() -> String {
    String::from("Hello world")
}

fn takes_and_give_back(s: String) -> String {
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &e) in bytes.iter().enumerate() {
        if e == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn change_word(s: &mut String) {
    s.push_str("Hello");
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn new_user(username: String, email: String) -> User {
    User {
        active: false,
        username,
        email,
        sign_in_count: 20,
    }
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    name: String,
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    let _name = &rectangle.name;
    println!("name is {}", rectangle.name);
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }
    fn new() -> Rectangle {
        Rectangle {
            name: String::from("长方形"),
            width: 10,
            height: 10,
        }
    }
}

enum Message {
    Quit,
    Move { x: u32, y: u32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Move { x, y } => println!("{} {}", x, y),
            Message::ChangeColor(x, y, z) => println!("{x}, {y}, {z}"),
            _ => {}
        }
    }
}
