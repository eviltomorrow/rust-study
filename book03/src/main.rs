use std::char;

fn main() {
    let v = build_vector();
    println!("{:?}", v);

    let v = build_vector2();
    println!("{:?}", v);

    let len = v.len();
    println!("{:?}", len);

    let c = b'3';
    println!("{}", c);

    let x = 100_u8.checked_add(200);
    assert_eq!(x, None);

    let x: f32 = -0_f32;
    let y: f32 = 0f32;
    assert_eq!(x, y);

    let x: i32 = 1000000000;
    let y = x as i16;
    println!("{}", y);

    let x = '\x2A';
    println!("{}", x);

    let x = '\u{CA0}';
    println!("{}", x);

    let x = '\u{D7FF}';
    println!("{}", x as i32);

    let x: u8 = 55;
    println!("{}", x as char);

    let x = char::from_u32(42);
    match x {
        Some(c) => println!("{}", c),
        None => println!("None"),
    }

    let text = "Hello world";
    let (head, tail) = text.split_at(5);
    println!(r"{}, {}", head, tail);

    let x: [i32; 2] = [1, 2];
    println!("{:?}", x);

    let mut x = [5, 4, 3, 2, 1];
    x.sort();
    println!("{:?}", x);

    let mut x = vec![0u32; 10];
    x.push(10);
    println!("{:?}", x);

    let mut v: Vec<i32> = Vec::with_capacity(10);
    v.push(10);
    let _ = v;

    let mut v = Vec::<i32>::with_capacity(20);
    v.push(10);
    println!("{:?}, {}", v, v.len());

    let v: Vec<i16> = (1..=5).collect();
    println!("{:?}", v);

    let mut palindrome = vec!["A", "B", "C", "D"];
    palindrome.reverse();
    println!("{:?}", palindrome);

    let mut v = [0u8; 3];
    move_array(v);
    v.reverse();

    println!(
        "In the room the women come and go,
Singing of Mount Abora"
    );

    println!(
        "In the room the women comd and go, \
        Singing of Mount Abora"
    );

    println!("In the room the women comd and go, \nSinging of Mount Abora");

    println!(
        r"In the room the women comd and go, \n
        Singing of Mount Abora"
    );

    println!(r##"I'm shepard," hello world"##,);

    let method = b"Get";
    println!("{:?}", method);
}

fn move_array(attr: [u8; 3]) {
    println!("{:?}", attr);
}

fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

fn build_vector2() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(16);
    v
}
