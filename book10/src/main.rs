use std::{cmp::Ordering, fmt::Debug};

fn main() {
    println!("Hello, world!");

    let result = compare(10, 20);
    println!("{:?}", result);

    use self::Pet::*;
    println!("{:?}", Orca);
    println!("{:?}", Giraffe);

    use std::mem::size_of;
    assert_eq!(size_of::<HttpStatus>(), 2);
    println!("{:?}", HttpStatus::NotFound);

    assert_eq!(HttpStatus::NotFound as i32, 404);

    http_status_from(200).unwrap();

    let s = TimeUnit::Seconds.pluar();
    println!("{}", s);

    let time_unit = time_unit_from("seconds");
    println!("{:?}", time_unit);

    let s = RoughTime::InTheFuture(TimeUnit::Days, 3);
    println!("{}", s.get_str());

    let s = RoughTime::JustNow;
    println!("{}", s.get_str());

    let s = RoughTime::InThePast(TimeUnit::Hours, 24);
    println!("{}", s.get_str());

    let next_1 = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "next_1",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));

    let next_2 = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "next_1",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));

    let tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "tree",
        left: next_1,
        right: next_2,
    }));

    println!("{:?}", tree.get_str());
}

fn compare(m: i32, n: i32) -> Ordering {
    if m > n {
        return Ordering::Greater;
    } else if m == n {
        return Ordering::Equal;
    } else {
        return Ordering::Less;
    }
}

#[derive(Debug)]
enum Pet {
    Orca,
    Giraffe,
}

#[derive(Debug)]
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

fn http_status_from(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None,
    }
}

fn time_unit_from(s: &str) -> TimeUnit {
    match s {
        "seconds" => TimeUnit::Seconds,
        "minutes" => TimeUnit::Minutes,
        "hours" => TimeUnit::Hours,
        "days" => TimeUnit::Days,
        "months" => TimeUnit::Months,
        "years" => TimeUnit::Years,
        _ => TimeUnit::Seconds,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

impl RoughTime {
    fn get_str(self) -> String {
        match self {
            Self::InTheFuture(unit, n) => format!("{} {} Later!", n, unit.pluar()),
            Self::JustNow => String::from("Just Now!"),
            Self::InThePast(unit, n) => format!("{}, {} Ago!", n, unit.pluar()),
        }
    }
}

#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

impl<T: Debug> BinaryTree<T> {
    fn get_str(&self) -> String {
        match self {
            BinaryTree::Empty => format!("empty"),
            BinaryTree::NonEmpty(data) => {
                let s = format!("{:?}", data.element);

                match data.left {
                    BinaryTree::Empty => {}
                    _ => {}
                }

                match data.right {
                    BinaryTree::Empty => {}
                    _ => {}
                }
                s
            }
        }
    }
}

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}
