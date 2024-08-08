fn main() {
    println!("Hello, world!");

    let u1 = User {
        active: true,
        username: String::from("shepard"),
        email: String::from("eviltomorrow@gmail.com"),
        sign_in_count: 1,
    };

    println!(
        "{}, {}, {}, {}",
        u1.active, u1.username, u1.email, u1.sign_in_count
    );

    let mut u2 = User {
        active: true,
        username: String::from("liarsa"),
        email: String::from("eviltomorrow@gmail.com"),
        sign_in_count: 2,
    };
    u2.active = false;

    let username = String::from("shepard");
    let email = String::from("eviltomorrow@163.com");
    let u2 = build_user(username, email);
    println!(
        "{}, {}, {}, {}",
        u2.active, u2.username, u2.email, u2.sign_in_count
    );

    let u3 = User { ..u2 };
    println!(
        "{}, {}, {}, {}",
        u3.active, u3.username, u3.email, u3.sign_in_count
    );
    println!("{}, {:?}", u2.active, "");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let (x, y, z) = (origin.0, origin.1, origin.2);
    println!("{}, {}, {}", black.0, black.1, black.2);
    println!("{}, {}, {}", x, y, z);

    let _sub = AlwaysEqual;

    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    let rect2 = Rectangle {
        width: dbg!(30),
        height: 20,
    };
    println!("{:#?}", rect2);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    let rect3 = Rectangle::new(10, 20);
    println!("{}", rect3.area());

    let sq = Rectangle::square(10);
    println!("The area of the rectangle is {} square pixels.", sq.area());

    let r1 = Rectangle {
        width: 30,
        height: 30,
    };
    let r2 = Rectangle::square(20);
    println!("The r1 can hold r2: {}", r1.can_hold(&r2));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rect: Rectangle) -> u32 {
    rect.width * rect.height
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
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
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;
