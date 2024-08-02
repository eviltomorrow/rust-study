use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let v: Vec<i32> = Vec::new();
    println!("{:#?}", v);

    let v = vec![1, 2, 3];
    println!("{:?}", v);
    print_type_of(&v);

    let v = 10;
    print_type_of(&v);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);
    println!("{:?}", &v);

    let ptr = &v;
    print_type_of(ptr);

    let third = v[3];
    println!("{:?}, {}", v, third);

    let third = v.get(10);
    match third {
        Some(third) => println!("{}", third),
        None => println!("None"),
    }
    println!("{:?}", v);
    let v1 = v;
    println!("{:?}", v1,);

    let mut v = Vec::new();
    v.push(String::from("one"));
    v.push(String::from("two"));
    v.push(String::from("three"));

    let two = &v[1];
    println!("{}", two);

    let third = &v[1];
    println!("{}", third);
    // println!("{:?},", v);
    // *two = String::from("");
    //

    for item in &v {
        println!("{}", item);
    }
    // println!("{:?}", v);

    let mut v = vec![1, 2, 3, 4, 5];
    for item in &mut v {
        *item += 10;
    }
    println!("{:?}", v);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(10),
        SpreadsheetCell::Float(10.0),
        SpreadsheetCell::Text(String::from("ten")),
    ];

    for item in &row {
        match item {
            SpreadsheetCell::Int(value) => println!("{}", value),
            SpreadsheetCell::Float(value) => println!("{}", value),
            SpreadsheetCell::Text(value) => println!("{}", value),
        }
    }

    let mut s = String::new();
    s.push('h');

    let s1 = String::from("Hello");
    let s2 = String::from("world");
    let _s3 = s1 + &s2;

    let data = "initial contents";
    let _s = data.to_string();

    let mut v = vec![String::from("Hello"), String::from("World")];
    for item in &mut v {
        item.push_str(" ok!");
    }
    println!("{:?}", v);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);
    let team_name = String::from("Blue");
    let value = scores.get(&team_name).copied().unwrap_or(0);
    println!("{:?}, {}", value, team_name);

    for (k, v) in &scores {
        println!("{}, {}", k, v);
    }

    let k1 = String::from("k1");
    let k2 = String::from("k2");
    let v1 = String::from("v1");
    let v2 = String::from("v2");
    let mut map = HashMap::new();
    map.insert(k1, v1);
    map.insert(k2, v2);
    println!("{:?}", map);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 20);
    scores.entry(String::from("Yellow")).or_insert(50);
    let s = scores.entry(String::from("Blue")).or_insert(50);
    *s = 100;

    println!("{:#?}", scores);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
