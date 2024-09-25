use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn main() {
    println!("Hello, world!");

    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "Many madrigals".to_string(),
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
            "A salt cellar".to_string(),
        ],
    );

    show(&mut table);

    let r = 10;
    let x = &r;
    println!("{}", *x);

    let aria = Anime {
        name: "Liarsa",
        bechdel_pass: true,
    };
    let aria_ref = &aria;
    println!("{}, {}", aria_ref.name, (*aria_ref).bechdel_pass);

    let mut v = vec![4, 3, 2, 1];
    v.sort();
    (&mut v).sort();
    println!("{:?}", v);

    let x = 10;
    let y = 20;
    let mut r = &x;
    println!("{}", r);
    r = &y;
    println!("{}", r);

    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 10, y: 20 };
    let p1_ref: &Point = &point;
    let p2_ref: &&Point = &p1_ref;
    let p3_ref = &p2_ref;
    println!("{}, {}", p3_ref.x, p3_ref.y);

    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);
    assert!(!std::ptr::eq(rrx, rry));

    let r = &factorial(6);
    println!("{}", r + &10);

    let x = 10;
    {
        let r;
        r = &x;
        assert_eq!(r, &10);
    }

    f(&10);
    f(&WORTH_POINTING_AT);
    unsafe {
        println!("{}", STASH);
    }

    let x = 10;
    g(&x);
    g1(&x);

    let v = vec![1, 2, 3, 4, 5];
    let c = smallest(&v);
    println!("{}", *c);

    let r = 10;
    {
        let s;
        s = S { r: &r };
        println!("{:?}, {}", s, s.r);
    }

    let s;
    {
        let r = 10;
        s = S { r: &r };
        println!("{}", s.r);
    }

    let f = Father {
        child: Child {
            name: &String::from("shepard"),
        },
    };
    println!("{:?}", f.child.name);

    let d = D { s: &S { r: &10 } };
    println!("{:?}", d.s);

    let x = 10;
    let r;
    {
        let y = 20;
        let c = C { x: &x, y: &y };
        r = c.x;
        println!("{}", c.y);
    }
    println!("{}", r,);

    let c = C { x: &10, y: &20 };
    let (r, c) = sum_r_xy(&10, c);
    println!("{}", r);
    println!("{:?}", c);

    let a = [0i32; 3];
    first_third(&a);

    let st = StringTable {
        elements: vec![
            String::from("hello1"),
            String::from("hello2"),
            String::from("hello3"),
        ],
    };

    let result_option = st.find_by_prefix("hello");
    println!("{:?}", result_option);

    let v = vec![4, 8, 19, 27, 34, 10];
    let r = &v;
    println!("{}", r[0]);
    let aside = v;
    println!("{:?}", aside);

    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];

    extend(&mut wave, &head);
    extend(&mut wave, &tail);
    println!("{:?}", wave);

    let mut x = 10;
    println!("{}", x);
    x = 20;
    let r = &x;
    println!("{}", r);
    println!("{}", x);

    // extend(&mut wave, &wave);
    //
    let mut v = (136, 139);
    let m = &mut v;
    let m0 = &mut m.0;
    *m0 = 137;
    let r0 = &m.0;

    println!("{}", r0);
}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}

fn first_third(point: &[i32; 3]) -> (&i32, &i32) {
    (&point[0], &point[1])
}

fn sum_r_xy<'a, 'b, 'c>(r: &'a i32, c: C<'b, 'c>) -> (i32, C<'b, 'c>) {
    (r + c.x + c.y, c)
}

#[derive(Debug)]
struct C<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

struct Father<'a> {
    child: Child<'a>,
}

struct Child<'a> {
    name: &'a String,
}

#[derive(Debug)]
struct S<'a> {
    r: &'a i32,
}

struct D<'a> {
    s: &'a S<'a>,
}

static mut STASH: &i32 = &128;

static WORTH_POINTING_AT: i32 = 1000;

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn g1<'a>(p: &'a i32) {
    println!("{}", p);
}

fn g(p: &i32) {
    println!("{}", p);
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

fn factorial(n: usize) -> usize {
    (1..n + 1).product()
}

struct Anime {
    name: &'static str,
    bechdel_pass: bool,
}

fn show(table: &mut Table) {
    for (artist, works) in table {
        println!("works by {}", artist);
        works.sort();
        for work in works {
            println!("  {}", work);
        }
    }
}
