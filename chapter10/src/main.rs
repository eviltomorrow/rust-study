use std::fmt::Display;

fn main() {
    println!("Hello, world!");

    let v1 = vec![1, 2, 3, 4, 5, 6];
    let v2 = vec![6, 5, 4, 3, 2, 1];
    println!("{}", large(&v1));
    println!("{}", large(&v2));

    let v1 = vec![1, 2, 3, 4, 5, 6, 7];
    println!("{}", large_i32(&v1));

    let v2 = vec!['c', 'b', 'a', 'e', 'f'];
    println!("{}", large_char(&v2));

    largest(&v1);

    let p = Point { x: 10, y: 20 };
    println!("x is {}, y is {}", p.x(), p.y);

    let p = Point {
        x: 10.0_f32,
        y: 10.0_f32,
    };
    println!("distance: {}", p.distance_from_origin());

    let p2 = Point2 { x: 10, y: 20.0 };
    p2.mixup(Point2 { x: 10, y: 10.0 });

    let p3 = Point2 { x: 10, y: 20 };
    let b = "10";
    let p4 = p3.mixup2(Point2 {
        x: String::from("a"),
        y: b,
    });
    println!("{}", p4.y);

    let s = Tweet {
        username: String::from("shepard"),
        content: String::from("hello world"),
    };
    println!("summarize is {}", s.summarize());

    let s = String::from("Shepard");
    println!("{}", s.summarize());

    notify(&s);
    notify2(&s);
    notify3(&s, &s);
    notify4(&s);
    notify5(&s);
    some_function(&s, &s);
    some_function2(&s, &s);

    let s1 = String::from("Hello world");
    let s2 = "shepard";

    let s3 = longest(&s1, s2);
    println!("{}", s3);
}

fn large(list: &[i32]) -> &i32 {
    let mut largest_number = &list[0];
    for number in list {
        if largest_number < number {
            largest_number = number;
        }
    }
    largest_number
}

fn large_i32(list: &[i32]) -> &i32 {
    let mut largest_number = &list[0];
    for number in list {
        if largest_number < number {
            largest_number = number;
        }
    }
    largest_number
}

fn large_char(list: &[char]) -> &char {
    let mut larget_char = &list[0];
    for number in list {
        if larget_char < number {
            larget_char = number;
        }
    }
    larget_char
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for e in list {
        if largest < e {
            largest = e;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

impl<X2, Y2> Point2<X2, Y2> {
    fn mixup2<X1, Y1>(self, other: Point2<X1, Y1>) -> Point2<X1, Y2> {
        Point2 {
            x: other.x,
            y: self.y,
        }
    }
}

trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    location: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, {}", self.location, self.content)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, {}", self.username, self.content)
    }
}

impl Summary for String {
    fn summarize(&self) -> String {
        format!("I am {}", self)
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

fn notify3<T: Summary>(item1: &T, item2: &T) {
    println!("Union news! {}, {}", item1.summarize(), item2.summarize());
}

fn notify4(item: &(impl Summary + Display)) {
    println!("Breaking new {}", item.summarize());
}

fn notify5<T: Summary + Display>(item: &T) {
    println!("Breaking new {}", item.summarize());
}

fn some_function<T: Display + Clone, U: Clone + Summary>(t: &T, u: &U) -> i32 {
    _ = t;
    _ = u;
    0
}

fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Clone,
    U: Summary,
{
    _ = t;
    _ = u;
    0
}

pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Pair { x: x, y: y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x > self.y {
            println!("The larget member is x = {}", self.x);
        } else {
            println!("The larget member is y = {}", self.y);
        }
    }
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
