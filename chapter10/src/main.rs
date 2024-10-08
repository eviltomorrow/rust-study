use std::fmt::{Debug, Display};

fn main() {
    println!("Hello, world!");

    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = &number_list[0];
    for number in &number_list {
        if largest < number {
            largest = number;
        }
    }
    println!("The largest is {:?}", largest);

    let number_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let largest = largest_i32(&number_list);
    println!("{}", largest);

    let char_list: [char; 3] = ['a', 'b', 'c'];
    let largest = largest_char(&char_list);
    println!("{}", largest);

    let largest = largest_char_i32(&char_list);
    println!("{}", largest);

    let p1 = Point { x: 10, y: 20 };
    println!("{:?}, {}, {}", p1, p1.x, p1.y);

    let p2 = Point2 {
        x: 10,
        y: String::from("H"),
    };
    println!("{:?}, {}, {}", p2, p2.x, p2.y);

    println!("{}", p1.x());

    let p3 = Point {
        x: 10.0_f32,
        y: 20.0_f32,
    };
    let distance = p3.distance_from_origin();
    println!("{}", distance);

    let p3 = Point3 { x: 10, y: 'c' };
    let p3 = p3.mixup(Point3 { x: (), y: () });
    println!("{:?}", p3);

    let tweeter = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!(
        "1 new tweet: {}, {}",
        tweeter.summarize(),
        tweeter.summarize2()
    );

    let r: i32 = 0;
    println!("{}", r);

    let r1 = String::from("A");
    let result;
    {
        let r2 = String::from("AB");
        result = longest(&r1, &r2);
        println!("{}", result);
    }
    // println!("{}", result);

    let r1 = String::from("A");
    {
        let result;
        let r2 = String::from("AB");
        result = longest(&r1, &r2);
        println!("{}", result);
    }

    let r1 = String::from("A");
    let result;
    {
        let r2 = String::from("AB");
        result = longest2(&r2, &r1);
        println!("{}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().unwrap();
    let i = ImportantExcerpt {
        name: "",
        part: first_sentence,
    };
    println!("{:?}, {}, {}", i, i.name, i.part);

    let result;
    {
        let r2 = String::from("AB");
        result = longest3(&r2);
        println!("{}", result);
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a, T> {
    name: T,
    part: &'a str,
}

fn longest3(x: &str) -> &str {
    x
}

fn longest2<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn largest_i32(list: &[i32]) -> &i32 {
    if list.len() == 0 {
        panic!("invalid list");
    }

    let mut largest = &list[0];
    for item in list {
        if largest < item {
            largest = item
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    if list.len() == 0 {
        panic!("invalid list");
    }

    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char_i32<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    if list.len() == 0 {
        panic!("invalid list");
    }

    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
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

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

#[derive(Debug)]
struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize2(&self) -> String {
        format!("default, {}", self.summarize())
    }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
    println!("{}, {}", item1.summarize(), item2.summarize());
}

pub fn notify4<T: Summary>(item1: &T, item2: &T) {
    println!("{}, {}", item1.summarize(), item2.summarize());
}

pub fn notify5(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify6<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {
    _ = t;
    _ = u;
}

pub fn some_function2<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    _ = t;
    _ = u;
}

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("h"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: true,
    }
}
