use rand::Rng;
// use std::io::{self, Write};
//
mod garden;

fn main() {
    println!("Hello, world!");

    chapter07::lib::println();
    let a = garden::vegetables::Asparagus {
        name: String::from("H"),
    };
    println!("{a:?}, {}", a.name);

    chapter07::hosting::add_to_waitlist();
    chapter07::front_of_house::hosting::add_to_waitlist();

    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("{}", secret_number);
}
