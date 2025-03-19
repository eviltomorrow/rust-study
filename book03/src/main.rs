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

    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    println!("{}, {}", head, tail);

    let temp = text.split_at(21);
    let head = temp.0;
    let tail = temp.1;
    println!("{}, {}", head, tail);

    let c = swap();
    println!("{:?}", c);

    let mut x = 10;
    let r = &x;
    println!("{}, {}", r, *r);

    let r = &mut x;
    *r = 20;
    println!("{}", x);

    let t = (12, "eggs");
    let b = Box::new(t);
    println!("{:?}", b);

    let lazy_caterer: [u32; 3] = [1, 2, 3];
    println!("{:?}", lazy_caterer);

    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
    println!("{:?}", taxonomy);

    let mut sieve = [true; 10];
    for i in 2..sieve.len() {
        if sieve[i] {
            let mut j = i * i;
            while j < sieve.len() {
                sieve[j] = false;
                j += i;
            }
        }
    }
    println!("{:?}", sieve);

    let mut chaos = [3, 5, 4, 1, 2, 6];
    chaos.sort();

    println!("{:?}", chaos);

    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    let mut c = new_pixel_buffer(4, 5);
    c.push(3);
    println!("{}, {}", c.len(), c.capacity());

    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, vec!["step", "on", "no", "pets"]);

    let v: Vec<u8> = (0..=5).collect();
    println!("{:?}", v);

    let mut palindrome = vec!["man", "plan", "canal", "panama"];
    palindrome.reverse();
    assert_eq!(palindrome, vec!["panama", "canal", "plan", "man"]);
}

fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
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

fn swap() -> (i32,) {
    (10,)
}
