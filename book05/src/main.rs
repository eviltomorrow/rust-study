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
