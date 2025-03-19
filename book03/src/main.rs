fn main() {
    let v1 = build_vector_1();
    println!("{:?}", v1);

    let v2 = build_vector_2();

    println!("{:?}", v2);

    let c = 'c';
    let d = 67u8;
    println!("{}", c as u8 == d);

    let c = b'C';
    let d = 67;
    println!("{}", c == d);

    let d = 10i16;
    println!("{}", d as f32);

    let d = 10.2_f64;
    println!("{}", d as i32);

    assert_eq!(1000_i16 as u8, 232u8);
    println!("{}", 1000i16 % 8);

    println!("{}", (-4i32).abs());
    println!("{}", i32::abs(-4));

    let mut i: i32 = 1;
    loop {
        // i = i.checked_mul(10).expect("overflow...");
        i = i.checked_mul(100).unwrap();
        println!("{}", i);

        break;
    }

    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);

    println!("{}", 500 * 500 % 2i32.pow(16));

    println!("{}", (2.0_f64).sqrt());

    let x = 10;
    if x < 100 {
        println!("{}", x);
    }

    assert_eq!(b'c', 99);
    assert_eq!('*' as i32, 42);
    assert_eq!(42 as char, '*');
}

fn build_vector_1() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

fn build_vector_2() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}
