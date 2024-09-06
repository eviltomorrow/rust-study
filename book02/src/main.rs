fn main() {
    pub fn answer() -> () {
        let a = 10;
        let b = 10;
        assert_eq!(20, a + b);
    }

    pub fn sum(x: i32, y: i32) -> i32 {
        x + y
    }

    answer();
    let z = sum(10, 10);
    assert_eq!(20, z);

    let a = [1, 2, 3];
    let b = &a;
    println!("{:?}, {:p}", a, b);

    let mut c = vec![1, 2, 3];
    let d = &mut c;
    d.push(10);
    println!("{:?}", d);

    let e = &42;
    assert_eq!(42, *e);

    let sum_value = math(sum, 10, 20);
    assert_eq!(30, sum_value);

    let attr = [0; init_len()];
    println!("{:?}", attr);

    let out = 42;

    fn add(i: i32, j: i32) -> i32 {
        i + j
    }

    let closure_annotated = |i: i32, j: i32| -> i32 { i + j + out };
    let closure_inferred = |i, j| i + j + out;

    let i = 1;
    let j = 2;
    assert_eq!(3, add(1, 2));
    assert_eq!(45, closure_annotated(i, j));
    assert_eq!(45, closure_inferred(i, j));

    let a = 2;
    let b = 3;
    assert_eq!(math1(|| a + b), 5);
    assert_eq!(math1(|| a * b), 6);

    let fn1 = two_times_impl();
    assert_eq!(fn1(2), 4);

    let n = 13;
    let big_n = if n < 10 && n > -10 { 10 * n } else { n / 2 };
    assert_eq!(big_n, 6);

    for n in 1..15 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let number = 42;

    match number {
        0 => println!("origin"),
        1..=3 => println!("All"),
        5 | 8 | 13 => println!("Bad luck"),
        n @ 42 => println!("{}", n),
        _ => println!("Common"),
    }

    let boolean = true;
    let mut binary = 0;
    if let true = boolean {
        binary = 1;
    }
    println!("{}", binary);

    let mut v = vec![1, 2, 3, 4, 5];
    loop {
        match v.pop() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }

    let mut v = vec![6, 7, 8, 9, 10];
    while let Some(x) = v.pop() {
        println!("{}", x);
    }

    let attr: [i32; 3] = [1, 2, 3];
    println!("{:?}", attr);

    let attr = [1, 2, 3];
    assert_eq!(1, attr[0]);

    assert_eq!((1..5), std::ops::Range { start: 1, end: 5 });
    assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5));

    let a1 = [1, 2, 3, 4, 5];
    let b1 = &a1;
    let b2 = &a1[0..];

    let c1 = [1, 2, 3, 4, 5];
    println!(
        "{}, {}, {:p}, {:p}, {:p}, {:p}",
        b1 == b2,
        a1 == c1,
        b1,
        b2,
        &a1,
        &c1
    );

    let truth: &'static str = "Rust是一门优雅的语言";
    let ptr = truth.as_ptr();
    let len = truth.len();

    assert_eq!(28, len);

    let s = unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8(slice)
    };
    assert_eq!(s, Ok(truth));

    let mut x = 10;
    let ptr_x = &mut x as *mut i32;

    let y = Box::new(20);
    let ptr_y = &*y as *const i32;

    unsafe { *ptr_x += *ptr_y }
    assert_eq!(x, 30);

    let num: Option<u32> = Some(32);
    match num {
        Some(x) => x,
        None => panic!("Hello world"),
    };

    let a = Number::One;
    match a {
        Number::Zero => println!("0"),
        Number::One => println!("1"),
        Number::Two => println!("2"),
    }

    println!("{:?}, {:?}", Number::Zero, Number::Two);
}

pub fn fizz_buzz(num: i32) -> String {
    if num % 15 == 0 {
        return "fizzbuzz".to_string();
    } else if num % 3 == 0 {
        return "fizz".to_string();
    } else if num % 5 == 0 {
        return "buzz".to_string();
    } else {
        return num.to_string();
    }
}

pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}

// #![feature(const_fn)]
const fn init_len() -> usize {
    let x = 5;
    return x;
}

fn math1<F: Fn() -> i32>(op: F) -> i32 {
    op()
}

fn two_times_impl() -> impl Fn(i32) -> i32 {
    let i = 2;
    move |j| j * i
}

pub fn while_true(x: i32) -> i32 {
    loop {
        return x;
    }
}

#[derive(Debug)]
enum Number {
    Zero,
    One,
    Two,
}

// #[derive(Debug)]
// enum Color {
//     Red = 0xff0000,
//     Green = 0x00ff00,
//     Blue = 0x0000ff,
// }

// #[derive(Debug)]
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }
