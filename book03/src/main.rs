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

    assert_eq!(false as i8, 0);
    assert_ne!(true as i8, 10);

    assert_eq!('*' as i32, 42);
    assert_eq!('*'.len_utf8(), 1);

    let t = ("hello", 4);
    const N: isize = 0;
    println!("{}, {}", t.0, N);

    let text = "Hello world";
    let (head, tail) = text.split_at(5);
    println!("{}, {}", head, tail);

    let temp = text.split_at(5);
    let head = temp.0;
    let tail = temp.1;
    println!("{}, {}", head, tail);

    let sum = (5 + 5,);
    println!("{:?}", sum);

    let x = 10;
    let p = &x;
    println!("{}, {}", p, *p);

    let t = (12, "eggs");
    let b = Box::new(t);
    println!("{:?}", b);

    let lazy_caterer: [u32; 6] = [1, 2, 3, 4, 5, 6];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
    println!("{:?}, {:?}", lazy_caterer, taxonomy);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    assert!(sieve[211]);

    const ARRAY_N: usize = 10;
    let array: [i32; ARRAY_N];
    array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", array);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    println!("{:?}", chaos);

    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    let v = new_pixel_buffer(2, 4);
    println!("{:?}", v);

    let buf = [0u8; 8];
    println!("{:?}", buf);

    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    println!("{:?}", pal);

    let mut v: Vec<i32> = (0..5).collect();
    v.push(10i32);

    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    println!("{:?}", v.capacity());

    let mut v = vec![10, 20, 30, 40, 50];
    v.insert(3, 35);
    v.remove(1);
    println!("{:?}", v);

    let mut v = vec!["H", "J"];
    v.pop();
    v.pop();
    assert_eq!(v.pop(), None);

    let mut languages: Vec<String> = std::env::args().skip(1).collect();
    for l in &mut languages {
        l.push('!');
    }
    println!("{:?}", languages);

    let v = vec![0.0f64, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;
    println!("{:?}, {:?}", sv, sa);

    print(&v);
    println!("{:?}", v.get(0));

    println!(
        "In the room the women come and go,
        Singing of Mount Abora!"
    );

    let s = "I am
 Liarsa";
    println!("{}", s);

    let s = "I am \n Liarsa";
    println!("{}", s);

    println!(
        "It was a bright, cold day in April, and \
        there wre four of us-- \
        more or less."
    );

    let default_install_path = r"C:\Program Files\Gorillas";
    println!("{}", default_install_path);

    let default_install_path = r#"C:\Program Files\Gorillas\"c""#;
    println!("{}", default_install_path);

    let c = b'c';
    let d = 'c';
    println!("{}, {}", c, d);

    let method = "GET";
    println!("{:?}", method);

    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    println!("{}, {}", noodles, oodles);
}

fn print(v: &[f64]) {
    for n in v {
        println!("{}", n);
    }
}

fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<i8> {
    vec![0; rows * cols]
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
