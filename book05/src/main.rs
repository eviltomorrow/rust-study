use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

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
            "The Calling of St. Mattew".to_string(),
        ],
    );

    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    show(&table);
    println!("{:#?}", table);

    let x = 10;
    let r = &x;
    assert!(*r == 10);

    let mut y = 20;
    let m = &mut y;
    *m += 20;
    assert!(*m == 40);

    struct Anime {
        name: &'static str,
        bechdel_pass: bool,
    }

    let aria = Anime {
        name: "hello",
        bechdel_pass: true,
    };

    let anime_ref = &aria;
    println!("{}, {}", (*anime_ref).name, anime_ref.bechdel_pass);

    let mut v = vec![1973, 1968];
    v.sort();
    (&mut v).sort();
    println!("{:?}", v);

    let x = 10;
    let y = 20;
    let mut r = &x;
    if true {
        r = &y;
    }

    println!("{}", r);

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 1000, y: 300 };
    let point_r: &Point = &point;
    let point_rr: &&Point = &point_r;
    let point_rrr: &&&Point = &point_rr;

    println!("{:?}, {}, {}", point_rrr, point_rrr.x, point_rrr.y);

    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);

    assert!(rx == ry);
    assert!(!std::ptr::eq(rx, ry));

    assert!(rx == *rrx);

    let r = &factorial(10);
    println!("{}", r);

    let x = 1;
    {
        let r = &x;
        println!("{}", r);
    }

    let r;
    {
        let x = 1;
        r = &x;
        println!("{}", r);
    }

    static WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT);

    unsafe {
        println!("{}", *STASH);
    }

    let x = 10;
    g(&x);

    let s;
    {
        let parabola = vec![1, 2, 3, 4, 5];
        s = smallest(&parabola);
        println!("{}", s);
    }

    let s;
    {
        let x = 10;
        s = S { r: &x };
        println!("{}", s.r);
    }

    let x = 10;
    let r;
    {
        let y = 20;
        {
            let t = T { x: &x, y: &y };
            r = t.x;
            println!("{}", t.y);
        }
    }
    println!("{}", r);

    let v = vec![1, 2, 3, 4, 5, 6, 7];
    let r = &v;
    println!("{:?}", r);
    let aside = v;
    println!("{:?}", aside);

    let v = vec![1, 2, 3, 4, 5];
    let r = &v[0];
    println!("{}", r);
}

struct T<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

struct S<'a> {
    r: &'a i32,
}

fn smallest<'a>(v: &'a [i32]) -> &'a i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

static mut STASH: &i32 = &10;

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn g<'a>(p: &'a i32) {
    println!("{}", p);
}

fn factorial(n: usize) -> usize {
    (1..n + 1).sum()
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}
