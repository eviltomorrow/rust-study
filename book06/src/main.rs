use std::collections::HashMap;

use rand::Rng;

fn main() {
    println!("Hello, world!");

    let msg = if get_true() {
        "s".to_string()
    } else {
        "o".to_string()
    };
    println!("{}", msg);

    {
        fn get_name() -> String {
            "H".to_string()
        }

        fn get_age() -> i32 {
            10
        }

        get_name();
        get_age();
    }

    match get_code() {
        0 => println!("OK"),
        1 => {
            println!("Not OK: 1");
        }
        2 => println!("Not OK: 2"),
        _ => println!("Not OK: default"),
    }

    let c = 10;
    let c = if let 20 = c { c + 30 } else { 0 };
    println!("{}", c);

    let c = 10;
    match c {
        10 => {
            println!("{}", 10)
        }
        _ => {
            println!("not {}", 10)
        }
    }

    match 10 {
        _ => println!("{}", 10),
    }

    let mut data = Vec::new();
    data.push("A".to_string());
    data.push("B".to_string());
    data.push("C".to_string());

    if data.len() == 3 {
        println!("{}", data.len());
    }
    println!("{:?}", data);

    for d in &data {
        println!("{:p}", d);
    }
    println!("{:?}", data);

    let mut params = HashMap::new();
    params.insert("name", Some("Hello".to_string()));

    let c = match params.get("name") {
        Some(name) => {
            println!("{:?}", name);
            "C".to_string()
        }
        None => {
            println!("Greetings, stranger.");

            "D".to_string()
        }
    };
    println!("{}", c);

    let mut n = 0;
    while get_true() {
        n += 1;
        if n > 10 {
            break;
        }
    }

    for i in 0..20 {
        println!("{}", i);
    }

    let mut strings = vec!["S".to_string(), "O".to_string(), "C".to_string()];
    for s in &mut strings {
        s.push('a');
        println!("{}", s);
    }
    println!("{}", strings.len());

    let answer = loop {
        if let Some(line) = next_line() {
            if line.starts_with("answer:") {
                break line;
            }
        } else {
            break "answer: nothing";
        }
    };
    println!("{}", answer);

    let mut i = 0;
    while i > 10 {
        i += 1;
    }

    'search: for i in 0..=20 {
        if i > 10 {
            break 'search;
        }
        println!("{}", i);
    }

    let sqrt = 'outer: loop {
        let n = next_number();
        println!("next number: {}", n);
        for i in 1.. {
            let square = i * i;
            if square == n {
                break 'outer i;
            }
            if square > n {
                break;
            }
        }
    };
    println!("sqrt: {}", sqrt);

    // never_back();
    let mut slice = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    quicksort(&mut slice);
    println!("{:?}", slice);

    never_back();
}

fn quicksort<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }

    let pivot_index = partition(slice);
    quicksort(&mut slice[..pivot_index]);
    quicksort(&mut slice[pivot_index + 1..]);
}

fn partition<T: Ord>(slice: &[T]) -> usize {
    return slice.len() / 2;
}

fn get_true() -> bool {
    true
}

fn get_code() -> i32 {
    100
}

fn next_line() -> Option<&'static str> {
    // Some("answer: hello world".to_string())
    None
}

fn next_number() -> i32 {
    let mut rng = rand::rng();
    rng.random_range(0..30)
}

fn never_back() -> i32 {
    let mut c = 10;
    loop {
        if c >= 100 {
            return 10;
        }
        c = c - 1;
    }
}
