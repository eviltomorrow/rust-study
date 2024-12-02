fn main() {
    let v = build_vector_1();
    println!("{:?}", v);

    let v = build_vector_2();
    println!("{:?}", v);

    println!("{}", b'x' == 65u8);

    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(2525_u16 as i16, 2525_i16);
    assert_eq!(-1_i16 as i32, -1_i32);
    assert_eq!(65535_u16 as i32, 65535_i32);

    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(65535_u32 as i16, -1_i16);
    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);

    assert_eq!(2_u16.pow(4), 16);
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(0b101101_u8.count_ones(), 4);

    let c = -10;
    use_i32(c);
    assert_eq!(c.abs(), 10);

    let mut i: i8 = 1;
    loop {
        i = i.checked_mul(10).expect("overflow i");
        println!("{}", i);
    }
}

fn use_i32(c: i32) {
    println!("{}", c);
}

fn build_vector_1() -> Vec<i16> {
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
