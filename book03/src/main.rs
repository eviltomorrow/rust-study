fn main() {
    println!("Hello, world!");

    let v = build_vector_1();
    println!("{:?}", v);

    let v = build_vector_2();
    println!("{:?}", v);

    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(2525_u16 as i16, 2525_i16);

    assert_eq!(-1_i16 as i32, -1_i32);
    assert_eq!(65535_u16 as i32, 65535_i32);

    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(250000i32 as u16, 53392);

    assert_eq!(2_u16.pow(4), 16);
    assert_eq!((-4i32).abs(), 4);

    println!("{}", i32::abs(-4));

    let mut i: i32 = 1;
    loop {
        i = i.checked_mul(10).expect("Wrong!");
        println!("{:?}", i);
        if i > 1000000 {
            break;
        }
    }

    assert_eq!(255_u8.overflowing_sub(2), (253, false));

    println!("{}", 2f64.sqrt());
}

fn build_vector_1() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10);
    v
}

fn build_vector_2() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v
}
