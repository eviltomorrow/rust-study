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
        if i >= 100 {
            break;
        }
        i = i.checked_mul(10).expect("overflow i");
    }

    let c = 10e2;
    println!("{}", c);
    println!("{}", f32::MAX);
    println!("{}", std::f32::consts::PI);

    assert_eq!(true as i32, 1);
    assert_eq!('*' as i8, 42);

    let c = std::char::from_u32(42);
    if let Some(val) = c {
        println!("{}", val);
    }

    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    let temp = text.split_at(21);
    let head = temp.0;
    let tail = temp.1;
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    let mut x = 10;
    let mut y = 20;
    swap::<i8>(&mut x, &mut y);

    let c = ("H",);
    println!("{:?}", c);

    let t = 10;
    let t_ref = &t;
    println!("{}, {}", t_ref, *t_ref);

    let t = (12, "eggs");
    let b = Box::new(t);
    println!("{:?}", b);

    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve: [bool; 10000] = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < sieve.len() {
                sieve[j] = false;
                j += i;
            }
        }
    }

    assert!(sieve[211]);
    assert!(!sieve[9876]);

    let c: [u8; 3] = [0; 3];
    println!("{:?}", c);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    println!("{:?}", chaos);

    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);
    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    let c = new_pixel_buffer(2, 2);
    println!("{:?}", c);

    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, vec!["step", "on", "no", "pets"]);

    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);

    let mut palindrome = vec!["A", "B", "C", "D"];
    palindrome.reverse();
    println!("{:?}", palindrome);

    let v = Vec::<i32>::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    let mut v = Vec::with_capacity(2);
    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.capacity(), 4);

    let mut v = vec![10, 20, 30, 40, 50];
    v.insert(0, 0);
    println!("{:?}", v);

    v.remove(2);
    println!("{:?}", v);

    let v = v.pop();
    if let Some(c) = v {
        println!("{}", c);
    }

    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, 0.707, 1.0, 0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;
    println!("{}", sv == sa);

    print(&sv[0..2]);

    let speec = "\"Ouch!\" said the well.\n";
    println!("{}", speec);

    println!(
        "In the room the women come and go,
        Singing of Mount Abora"
    );

    println!(
        r"In the room the women come and go, \
        Singing of Mount Abora"
    );

    println!(r###"In the "room" the women come and \go."###);

    let method = b"GET";
    println!("{:?}", method);

    let method = "GET";
    assert_eq!(method.as_bytes(), b"GET");

    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    println!("{}", oodles);

    let s = String::from("成功");
    println!("{}", &s[3..]);

    println!("{}", s.chars().count());
}

fn print(n: &[f64]) {
    for e in n {
        println!("{}", e);
    }
}

fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}

fn swap<T>(x: &mut T, y: &mut T) -> () {
    _ = x;
    _ = y;
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
