use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");

    let result = compare(10, 20);
    println!("{:?}", result);

    use self::Pet::*;
    println!("{:?}", Orca);
    println!("{:?}", Giraffe);

    use std::mem::size_of;
    assert_eq!(size_of::<HttpStatus>(), 2);
    println!("{:?}", HttpStatus::NotFound);

    assert_eq!(HttpStatus::NotFound as i32, 404);
}

fn compare(m: i32, n: i32) -> Ordering {
    if m > n {
        return Ordering::Greater;
    } else if m == n {
        return Ordering::Equal;
    } else {
        return Ordering::Less;
    }
}

#[derive(Debug)]
enum Pet {
    Orca,
    Giraffe,
}

#[derive(Debug)]
enum HttpStatus {
    NotFound = 404,
}
