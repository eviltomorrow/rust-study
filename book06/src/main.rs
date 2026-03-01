fn main() {
    println!("Hello, world!");

    let ok = true;
    let name;
    if ok {
        name = "A";
    } else {
        name = "B";
    }
    println!("{}", name);

    let c = match get_code() {
        1 => "A",
        10 => "B",
        _ => "C",
    };
    println!("{:?}", c);

    let c = if ok { 10 } else { 20 };
    println!("{}", c);
}

fn get_code() -> i32 {
    10
}
