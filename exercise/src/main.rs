fn main() {
    println!("Hello, world!");
    let mut x = 20;
    x = x + 20;
    println!("x is {x}");

    say();
    say_something("shepard");
    println!("return {}", return_something());
    println!("{}", THREE_HOURS_SECONDS);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

    let a = 10.0;
    let b: f64 = 20.0;
    let sum = a + b;
    println!("sum is {}", sum);
    let sum = a * b;
    println!("sum is {}", sum);

    let c: f64 = 30.0;
    println!("sum is {}", a + c);

    let t: bool = false;
    if !t {
        println!("t is {}", t);
    }

    let mut c = 'a';
    println!("c is {c}");
    c = 'b';
    println!("c is {}", c);
    c = '你';
    println!("c is {}", c);

    let tup = (3, 2.0, 'c', true);
    println!("tup is {:#?}", tup);

    let a = [1, 2, 3];
    println!("{:?}", a);
    println!("{:?}", a);

    for e in a {
        println!("e is {:?}", e);
    }

    let mut count = 0;
    loop {
        count += 1;
        if count >= 10 {
            break;
        }
        println!("count-{}", count);
    }

    while count > 5 {
        count -= 1;
        println!("count-{count}");
    }

    let s1 = String::from("world");
    takes_ownership(s1);
    // println!("{s1}");

    let x = 5;
    make_copy(x);
    println!("{x}");

    let s = gives_ownership();
    let s = takes_and_give_back(s);
    println!("{s}");

    let (s, len) = calculate_length(s);
    println!("s is {s}, len is {len}");

    let l = calculate_length2(&s);
    println!("s is {s}, len is {l}");

    let loc = first_word(&s);
    println!("loc is {loc}");

    let hello = first_word2(&s);
    println!("hello is {hello}");

    let mut s = String::from("");
    s.push_str("a");
    println!("{s}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:#?}", &a[1..2]);

    let mut s = String::from("Hi");
    let a = &mut s;
    change_word(a);
    let b = &s;
    println!("{}",  b);
}

const THREE_HOURS_SECONDS: i64 = 32 * 60 * 60;

fn say() {
    println!("hello world");
}

fn say_something(world: &str) {
    println!("hello {}", world);
}

fn return_something() -> String {
    String::from("Hello world")
}

fn takes_ownership(s: String) {
    println!("{s}");
}

fn make_copy(x: i32) {
    println!("{x}");
}

fn gives_ownership() -> String {
    let s = String::from("Hello world");
    s
}

fn takes_and_give_back(s: String) -> String {
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    let l = s.len();
    l
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &e) in bytes.iter().enumerate() {
        if e == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn change_word(s: &mut String) {
    s.push_str("Hello");
}
