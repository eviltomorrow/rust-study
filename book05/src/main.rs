use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn main() {
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
            "Perseus with the head os Medusa".to_string(),
            "A salt cellar".to_string(),
        ],
    );

    sort_works(&mut table);
    show(&table);

    let x = 10;
    let r = &x;
    assert!(*r == 10);

    let mut y = 32;
    let m = &mut y;
    *m += 2;
    println!("{}", m);

    struct Anime {
        name: &'static str,
    }
    let aria = Anime {
        name: "Aria: The Animation",
    };
    let aria_ref = &aria;
    assert_eq!(aria_ref.name, "Aria: The Animation");
    assert_eq!((*aria_ref).name, "Aria: The Animation");

    let r = &factor(6);
    assert_eq!(r + &1009, 1729);

    let x = 10;
    let y = 20;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;
    assert_eq!(*rrx + *rry, 30);

    f(&10);
    // println!("{}", STASH);
}

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

static mut STASH: &i32 = &128;

fn factor(n: usize) -> usize {
    (1..n + 1).product()
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_, works) in table {
        works.sort();
    }
}
