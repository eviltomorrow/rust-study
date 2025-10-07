use std::collections::HashMap;

fn main() {
    let m = 10;
    let n = 20;

    let result = compare(n, m);
    println!("{:?}", result);

    assert_eq!(size_of::<Ordering>(), 1);
    assert_eq!(Ordering::Equal as i8, 1);

    let n = 20;
    let http_status_result = http_status_from_u32(n);
    let http_status = if let Some(data) = http_status_result {
        data
    } else {
        HttpStatus::Ok
    };
    println!("{:?}", http_status);

    let t = new_timeunit("Hours");
    let r1 = t.plural();
    let r2 = t.singular();
    println!("{}, {}", r1, r2);

    const ORIGIN: Point3d = Point3d { x: 0, y: 0, z: 0 };
    let unit_sphere = Shape::Sphere {
        center: ORIGIN,
        radius: 1.0,
    };
    println!("{:?}", unit_sphere);

    println!("{}", std::mem::size_of::<HashMap<i32, i32>>());

    let feature = RoughTime::InTheFuture(TimeUnit::Days, 10);
    let s = rough_time_to_english(feature);
    println!("{}", s);

    let s = String::from("c");
    let mut r: Result<String, &str> = Ok(s);
    match r {
        Ok(ref mut s) => {
            s.push_str("!");
            println!("{}", s);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
    println!("{:?}", r);

    let p = &Point3d { x: 0, y: 0, z: 0 };
    match p.center() {
        &Point3d { x, y, z } => println!("{}, {}, {}", x, y, z),
    }

    let friend = Person {
        name: String::from("Nike"),
        car: &Car {
            engine: String::from("XC60"),
        },
    };

    match friend.borrow_car() {
        Some(&Car { ref engine }) => {
            println!("{}", engine);
        }
        None => {
            let name = &friend.name;
            println!("{}", name);
        }
    }
    println!("{:?}", friend);

    let n = 10;
    match n {
        ..100 => println!("0..100"),
        _ => println!("not in 0..100"),
    }

    let t = (1, 2);
    match t {
        c @ (1, 2) => {
            println!("{:?}", c.0);
        }
        _ => println!("{}", "null"),
    }
}

#[derive(Debug)]
enum Ordering {
    Less,
    Equal,
    Greater,
}

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        return Ordering::Less;
    }
    if n > m {
        return Ordering::Greater;
    }
    Ordering::Equal
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn singular(self) -> &'static str {
        self.plural().trim_end_matches("s")
    }
}

fn new_timeunit(c: &'static str) -> TimeUnit {
    match c {
        "Seconds" => TimeUnit::Seconds,
        "Minutes" => TimeUnit::Minutes,
        "Hours" => TimeUnit::Hours,
        "Days" => TimeUnit::Days,
        "Months" => TimeUnit::Months,
        "Years" => TimeUnit::Years,
        _ => TimeUnit::Days,
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

#[derive(Debug)]
#[allow(dead_code)]
struct Point3d {
    x: u32,
    y: u32,
    z: u32,
}

impl Point3d {
    fn center(&self) -> &Self {
        self
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum Shape {
    Sphere { center: Point3d, radius: f32 },
    Cuboid { corner1: Point3d, corner2: Point3d },
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InTheFuture(units, mut count) => {
            count = 20;
            format!("{} {} ago", count, units.plural())
        }
        RoughTime::JustNow => format!("Just now"),
        RoughTime::InThePast(units, 1) => format!("{} {} from now", 1, units.singular()),
        RoughTime::InThePast(units, count) => format!("{} {} from now", count, units.plural()),
    }
}

#[derive(Debug)]
struct Car {
    engine: String,
}

#[derive(Debug)]
struct Person<'a> {
    name: String,
    car: &'a Car,
}

impl<'a> Person<'a> {
    fn borrow_car(&self) -> Option<&'a Car> {
        Some(self.car)
    }
}
