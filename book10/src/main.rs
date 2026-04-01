fn main() {
    println!("Hello, world!");

    let m = 10;
    let n = 20;

    let result = compare(n, m);
    match result {
        Ordering::Equal => println!("equal"),
        Ordering::Greater => println!("greater"),
        Ordering::Less => println!("less"),
    }

    assert_eq!(size_of::<Ordering>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2);
    assert_eq!(HttpStatus::Ok as i32, 200);

    let n = 200;
    let status = http_status_from_u32(n);
    println!("{:?}", status);

    let code = if let Some(HttpStatus::Ok) = status {
        200
    } else {
        0
    };
    println!("{}", code);

    let s = TimeUnit::Hours.sigular();
    println!("{}", s);

    let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
    println!("{:?}", four_score_and_seven_years_ago);

    let unit_sphere = Shape::Sphere {
        center: ORIGIN,
        radius: 1.0,
    };
    println!("{:?}", unit_sphere);

    use enum_impl;
    let s = enum_impl::rough_time_to_english(RoughTime::InThePast(TimeUnit::Hours, 3));
    println!("{:?}", s);

    let meadow = Meadow {};
    match meadow.count_rabbits() {
        0 => {}
        1 => println!("A rabbit is nosing around in the clover."),
        n => println!("There are {} rabbits hopping about in the meadow", n),
    }

    let settings = Settings {};
    let calendar = match settings.get_string("calendar") {
        "gregorian" => Calendar::Gregorian,
        "chinese" => Calendar::Chinese,
        "ethiopian" => Calendar::Ethiopian,
        _ => panic!("not found"),
    };
    println!("{:?}", calendar);

    let s = describe_point(10, 20);
    println!("{:?}", s);

    let balloon = Balloon {
        location: Point { x: 30, y: 40 },
    };
    match balloon.location {
        Point { x: 0, y: height } => println!("straight up {} meters", height),
        Point { x, y, .. } => println!("at {}m, {}m", x, y),
    }

    hsl_to_rgp([0u8; 3]);
    hsl_to_rgp([0, 0, 0]);

    let c: [u8; 3] = [0; 3];
    hsl_to_rgp(c);

    greet_people(&["A", "B"]);

    let mut account = Account {
        name: String::from("Shepard"),
        language: String::from("Chinese"),
        id: 0,
    };

    match account {
        Account {
            ref name,
            ref mut language,
            ..
        } => {
            println!("{}, {}", name, language);
            language.push_str(" Hi");
            print_account(&account);
        }
    }

    let s = Shape::Sphere {
        center: Point3d { x: 0, y: 0, z: 0 },
        radius: 1.0,
    };

    match s.center() {
        &Point3d { x, y, z } => {
            println!("{}, {}, {}", x, y, z);
        }
    }

    let friend = Friend {
        car: Car { engine: Engine {} },
    };
    match friend.borrow_car() {
        Some(&Car { ref engine, .. }) => {
            println!("engins is: {:?}", engine);
        }
        None => {
            println!("poor!");
        }
    }

    let account = Account {
        name: String::from("Hello"),
        language: String::from("English"),
        id: 10,
    };

    let Account { name, language, id } = account;
    println!("{}, {}, {}", name, language, id);
}

enum Ordering {
    Less,
    Equal,
    Greater,
}

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Ordering::Less
    } else if n > m {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None,
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    fn pluar(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn sigular(self) -> &'static str {
        self.pluar().trim_end_matches('s')
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

const ORIGIN: Point3d = Point3d { x: 0, y: 0, z: 0 };

#[derive(Debug)]
#[allow(dead_code)]
struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Shape {
    Sphere { center: Point3d, radius: f32 },
    Cubold { corner1: Point3d, corner2: Point3d },
}

impl Shape {
    fn center(&self) -> &Point3d {
        match self {
            Self::Sphere { center, .. } => center,
            _ => panic!("Error"),
        }
    }
}

#[allow(dead_code)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

#[allow(dead_code)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

pub mod enum_impl;

struct Meadow {}

impl Meadow {
    fn count_rabbits(&self) -> u32 {
        10
    }
}

struct Settings {}

impl Settings {
    fn get_string(&self, _: &str) -> &'static str {
        "chinese"
    }
}

#[derive(Debug)]
enum Calendar {
    Gregorian,
    Chinese,
    Ethiopian,
}

fn describe_point(x: i32, y: i32) -> &'static str {
    use std::cmp::Ordering::*;

    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "on the x axis",
        (Equal, _) => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        _ => "somewhere else",
    }
}

struct Balloon {
    location: Point,
}

struct Point {
    x: i32,
    y: i32,
}

fn hsl_to_rgp(hsl: [u8; 3]) -> [u8; 3] {
    match hsl {
        [_, _, 0] => [0, 0, 0],
        [_, _, 255] => [255, 255, 255],
        _ => [0, 0, 0],
    }
}

fn greet_people(names: &[&str]) {
    match names {
        [] => {
            println!("Hello, nobody.")
        }
        [a] => {
            println!("Hello, {}.", a)
        }
        [a, b] => {
            println!("Hello, {} and {}", a, b)
        }
        [a, .., b] => {
            println!("Hello, everyone from {} to {}.", a, b)
        }
    }
}

#[derive(Debug)]
struct Account {
    name: String,
    language: String,
    id: i32,
}

fn print_account(account: &Account) {
    println!("{:?}, {}", account, account.id);
}

struct Friend {
    car: Car,
}

impl Friend {
    fn borrow_car(&self) -> Option<&Car> {
        Some(&self.car)
    }
}

#[derive(Debug)]
struct Engine {}

struct Car {
    engine: Engine,
}
