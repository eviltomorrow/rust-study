fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = x;
    println!("{}, {}", x, y);

    let s1 = String::from("Hello world");
    let s2 = s1.clone();
    println!("{}, {}", s1, s2);

    let s = String::from("Hello world");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s = gives_ownership();
    let s2 = String::from("Hello world");
    let s3 = takes_and_give_back(s2);
    println!("{}, {}", s, s3);

    calculate_length(s3);

    let s = String::from("Hello world");
    let len = calculate_length_2(&s);
    println!("{s}, {len}, {}, {1}", s, len);

    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}", s);
    let r1 = &mut s;
    println!("{}", r1);
    let r2 = &mut s;
    println!("{}", r2);

    let mut s = String::from("hello world");
    {
        let r1 = &mut s;
        println!("{}", r1);
    }
    let r2 = &mut s;
    println!("{}", r2);

    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    let s = String::from("Hello world");
    let size = first_word(&s);
    println!("{size}");

    let s = String::from("Hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}, {}", hello, world);

    let s = String::from("Hello world");
    let len = s.len();

    let slice = &s[..len];
    println!("{}", slice);

    let s = first_word_2(&s);
    println!("{}", s);

    let s = first_word_3(&s);
    println!("{}", s);

    let s = "hello world";
    let s = first_word_3(&s);
    println!("{}", s);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello world");
    some_string
}

fn takes_and_give_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn calculate_length_2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", World");
}

// fn dangle_string() -> &String {
//     let s = String::from("Hello");
//     &s
// }

fn _nodangle_string() -> String {
    String::from("Hello")
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word_3(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
