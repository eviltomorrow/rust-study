use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );

    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );

    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );
    sort_works(&mut table);
    show_ref(&table);
    show(table);

    let x = 10;
    let r = &x;
    assert!(*r == 10);

    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert!(*m == 64);
    println!("{}", m);

    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };
    let aria_ref = &aria;
    assert_eq!(aria_ref.name, "Aria: The Animation");
    assert_eq!(aria_ref.bechdel_pass, true);

    let mut v = vec![3, 2, 1];
    v.sort();
    (&mut v).sort();
    println!("{:?}", v);

    let x = 10;
    let y = 20;
    let mut r = &x;
    if true {
        r = &y;
    }
    println!("{}", *r);

    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    assert_eq!(rrr.y, 729);
    println!("{}", rr.x);

    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;
    assert!(rrx <= rry);
    assert!(rrx == rry);

    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);

    let r;
    {
        let x = 1;
        r = &x;
        assert_eq!(*r, 1);
    }

    let x = 1;
    {
        let r;
        r = &x;
        println!("{}", r);
    }

    f_static(&10);

    unsafe {
        println!("{:?}", *STASH);
    }
    f(WORTH_POINTSING_AT);

    let x = 10;
    g(&x);

    let x = [1, 2, 3, 4, 5];
    let y = smallest(&x);
    println!("{}", y);

    smallest_n(&x);
    smallest_m(&x);
    let v = smallest_k(&x);
    println!("{}", v);

    let x = 10;
    let s = S { r: &x };
    println!("{:?}, {}", s, s.r);

    let d = D { s: S { r: &10 } };
    println!("{:?}, {:?}", d, d.s);

    let e = E { x: &10, y: &20 };
    println!("{:?}", e);

    let x = 10;
    {
        let y = 20;
        let e = E { x: &x, y: &y };
        println!("{:?}", e);
    }

    let x = 10;
    let y = 20;
    let e = E { x: &x, y: &y };
    println!("{:?} {} {}", e, e.x, e.y);

    let x = 10;
    let y = 20;
    f1(&x, &y);
    f2(&x, &y);

    let x = 10;
    {
        let t = 20;
        f1(&x, &t);
        f2(&x, &t);
    }
    f1(&x, &y);
    f3(&x, &y);

    sum_r_xy_1(&x, &e);
    sum_r_xy_2(&x, &e);
    println!("{}", e.sum(&10));

    let v = vec![1, 2, 3, 4, 5, 6];
    {
        let r = &v;
        r[0];
    }
    let aside = v;
    println!("{:?}", aside);

    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];

    extend(&mut wave, &head[..]);
    extend(&mut wave, &tail);
    println!("{:?}", wave);

    let mut wave = vec![1, 2, 3];
    let r = &mut wave;
    r[0] = 4;
    println!("{:?}", wave);
}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for e in slice {
        vec.push(*e);
    }
}

fn sum_r_xy_1(r: &i32, e: &E) -> i32 {
    r + e.x + e.y
}

fn sum_r_xy_2<'c, 'a, 'b>(r: &'c i32, e: &E<'a, 'b>) -> i32 {
    r + e.x + e.y
}

fn f1<'a, 'b>(x: &'a i32, _y: &'b i32) -> &'a i32 {
    x
}

fn f2<'a>(x: &'a i32, _y: &'a i32) -> &'a i32 {
    x
}

fn f3<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y { x } else { y }
}

#[derive(Debug)]
struct E<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

impl<'a, 'b> E<'a, 'b> {
    fn sum(&self, _another: &i32) -> &i32 {
        self.x
    }
}

#[derive(Debug)]
struct S<'a> {
    r: &'a i32,
}

#[derive(Debug)]
struct D<'a> {
    s: S<'a>,
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if r < s {
            s = r;
        }
    }
    s
}

fn smallest_n(v: &[i32]) {
    for n in &v[..] {
        println!("{}", n);
    }
}

fn smallest_m(v: &[i32]) {
    for n in v {
        println!("{}", n);
    }
}

fn smallest_k<'a, 'b>(v: &'a [i32]) -> &'b i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if r < s {
            s = r;
        }
    }
    &10
}

static mut STASH: &i32 = &128;
static WORTH_POINTSING_AT: &i32 = &8;

fn f_static(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn f(p: &i32) {
    println!("{}", p);
}

fn g<'a>(p: &'a i32) {
    println!("{}", p);
}

fn factorial(n: usize) -> usize {
    (1..n + 1).product()
}

struct Point {
    x: i32,
    y: i32,
}

struct Anime {
    name: &'static str,
    bechdel_pass: bool,
}

type Table = HashMap<String, Vec<String>>;

fn show(table: Table) {
    for (artist, works) in table {
        println!("works by {}: ", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn show_ref(table: &Table) {
    for (artist, works) in table {
        println!("works by {}: ", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}
