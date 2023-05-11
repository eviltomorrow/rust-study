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

    let v3 = vec![1, 2, 3, 4, 5, 6];
    println!("{}", largest(&v3));

    use aggregator::Tweet;
    let tweet = Tweet {
        username: String::from("hello world"),
        conent: String::from("this is shepard"),
    };
    let data = tweet.summarize();
    println!("data is {}", data);
    let data = tweet.summarize_default();
    println!("data is {}", data);

    notify(&tweet);
    notify3(&tweet);
    notify4(&tweet);
    notify2(tweet);

    let s = returns_summarizable();
    println!("{}", s.summarize());

    let s1 = String::from("hello");
    let s2 = "hello world";
    let s3 = longest(&s1, &s2);
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
    let mut largest_value = &list[0];
    for value in list {
        if largest_value < value {
            largest_value = value
        }
    }
    largest_value
}

pub fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

pub fn notify2(item: impl Summary) {
    println!("{}", item.summarize());
}

pub fn notify3<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

pub fn notify4<T>(item: &T)
where
    T: Summary,
{
    println!("{}", item.summarize());
}

pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize_default(&self) -> String {
        String::from("Read more...")
    }
}

mod aggregator {
    use crate::Summary;

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, {}, {}", self.headline, self.location, self.content)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub conent: String,
    }
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}, {}", self.username, self.conent)
        }
    }
}

fn returns_summarizable() -> impl Summary {
    use aggregator::Tweet;
    Tweet {
        username: "shepard".to_string(),
        conent: String::from("Hello world"),
    }
}

// fn returns_summarizable2(flag: bool) -> impl Summary{
//     use aggregator::{NewsArticle, Tweet};
//     if flag {
//         NewsArticle{
//             headline: String::from("h"),
//             location: String::from("b"),
//             content: String::from("c"),
//         }
//     }else{
//         Tweet{
//             username: String::from("a"),
//             conent: String::from("b"),
//         }
//     }
// }

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
