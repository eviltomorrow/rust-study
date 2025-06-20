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
