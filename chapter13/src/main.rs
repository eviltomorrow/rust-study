// use std::{thread, time::Duration};

use std::thread;

fn main() {
    println!("Hello world");

    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} get {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} get {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly ...");
        // thread::sleep(Duration::from_secs(1));
        num + 10
    };

    let result = expensive_closure(10);
    println!("result is: {}", result);

    let result = add_one_v1(10);
    println!("result is: {}", result);

    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    println!("result is: {}", add_one_v2(10));

    let add_one_v3 = |x| x + 1;
    println!("result is :{}", add_one_v3(10));

    let f = add_one_v1;
    println!("result is: {}", f(10));

    let example_closure = |x| x;
    let s = example_closure(String::from("Hello"));
    println!("{}", s);

    let list = vec![1, 2, 3, 4];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After  calling closure: {list:?}");

    let only_borrows2 = || {
        for c in &list {
            println!("borrow: {}", c);
        }
    };
    only_borrows2();

    let just_move = move |list: Vec<i32>| {
        for c in list {
            println!("move: {c}");
        }
    };
    just_move(list);

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(10);
    borrows_mutably();
    println!("After  calling closure: {:?}", list);

    let list = vec![1, 2, 3, 4, 5];
    println!("Before defining closure: {:?}", list);

    let t = thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
    println!("{:?}", t);
}

fn add_one_v1(x: u32) -> u32 {
    x + 1
}

#[derive(Debug, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

#[derive(Debug)]
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
