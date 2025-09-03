use std::i32;

fn main() {
    println!("Hello world");

    let s = "a";
    println!("{}", s);

    let v = build_vector();
    println!("{:?}", v);

    let v = build_vector_2();
    println!("{:?}", v);

    let c = 'c';
    println!("{}", c);

    let c = b'c';
    println!("{}", c);

    let c = b'\'';
    println!("{}", c);

    let c = b'\n';
    println!("{}", c);

    let c = b'\x1b';
    println!("{}", c);

    assert_eq!(10_i8 as i16, 10_i16);
    assert_eq!(-1_i16 as i32, -1_i32);
    assert_eq!(65535_u16 as i32, 65535_i32);

    assert_eq!(1000_i16 as u8, 232_u8);
    let c = 1000 % 2i16.pow(8);
    println!("{}", c);

    assert_eq!(2_u16.pow(4), 16);
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(0b00101_u8.count_ones(), 2);
    assert_eq!(i32::abs(-4), 4);

    println!("{}", std::i32::MAX);

    // check calculate
    let mut i: i32 = 1;
    loop {
        i = i.checked_mul(10).expect("multiplication overflowed");
        if i > 100 {
            break;
        }
    }

    let i: i32 = 10;
    if let Some(c) = i.checked_mul(10) {
        println!("{}", c);
    }
    assert_eq!(10_u8.checked_add(30), Some(40));

    // loopback
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);

    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_mul(10), -32768);

    // overflowing
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));

    println!("{}", (2.0_f64).sqrt());

    assert_eq!(false as i32, 0);
    assert_eq!(true as u8, 1);

    let c = '\x2A';
    println!("{}", c);

    let c = '\u{CA0}';
    println!("{}_{}", c, c);

    let c: u8 = 65;
    assert_eq!(c as char, 'A');

    assert_eq!('*' as i32, 42);

    let c: u32 = 1200;
    let result = std::char::from_u32(c);
    println!("{:?}", result);

    let result = char::from_u32(10);
    println!("{:?}", result);

    let tuple = ("Hello world", 100);
    println!("{}, {}", tuple.0, tuple.1);

    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    println!("{}, {}", head, tail);

    let t = ("Hello",);
    println!("{}", t.0);

    let x = 10;
    let r = &x;
    println!("{}, {}", r, *r);

    let rr = &r;
    println!("{}", rr);

    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
    let default_value = [10; 3];

    assert_eq!(default_value.len(), 3);
    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut chaos = [3, 4, 5, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);

    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;
    println!("{:?}, {:?}", sv, sa);

    print_slice(sv);
    print_slice(sa);

    print_slice(&sv[0..2]);
    print_slice(&v[1..3]);

    println!(
        "In the room the women come and go,
        Singing of Mount Abora"
    );

    println!(
        "It was a bright, cold day in April, and \
        there were four of us -- \
        more or less"
    );

    println!("C:\\Program Files\\Gorillas");
    println!(r"C:\Program FIles\Gorillas");

    println!(
        "Line \
        New line"
    );

    println!(
        r###"C:\Program""" \
        Files"###
    );

    let s1 = "H".to_string();
    let s2 = "I".to_string();
    println!("{}", s1 < s2);
}

fn print_slice(n: &[f64]) {
    for elt in n {
        println!("{}", elt);
    }
}

fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20_i16);
    v
}

fn build_vector_2() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}
