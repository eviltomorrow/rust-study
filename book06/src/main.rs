fn main() {
    println!("Hello, world!");

    let a = 10;
    let b = 20;
    let status = if a > b {
        "OK".to_string()
    } else {
        "Not OK".to_string()
    };
    println!("{:?}", status);

    let sqrt = 'outer: loop {
        let n = 25;
        for i in 1.. {
            let square = i * i;
            if square == n {
                break 'outer i;
            }
            if square > n {
                break;
            }
        }
    };
    println!("{}", sqrt);

    let v = get_val();
    println!("{}", v);
}

fn get_val() -> i32 {
    let a = 10;
    let mut b = 1;
    let c = true;
    while get_bool() {
        if b > a {
            return 100;
        }
        b = b + 1;
    }
}

fn get_bool() -> bool {
    true
}
