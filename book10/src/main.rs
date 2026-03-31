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
