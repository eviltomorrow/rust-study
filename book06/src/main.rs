use http::StatusCode;

fn main() {
    println!("Hello, world!");

    let x = 10;
    let status = if x > 10 {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };
    println!("{:?}", status);

    let v = get_some();
    println!(
        "This is {:?}",
        match v {
            Some(c) => c,
            None => 0,
        }
    );

    if x < 10 {
        println!("");
    }

    let c = if x < 10 {
        10;
    };
    println!("{:?}", c);

    let c = if let Some(val) = get_some() { val } else { 50 };
    println!("{}", c);

    for i in 0..2 {
        println!("{}", i);
    }

    let answer = loop {
        if let Some(s) = next_line() {
            if s.starts_with("answer") {
                break s;
            }
        } else {
            break String::from("");
        }
    };
    println!("{}", answer);

    'search: for i in 1..20 {
        if i == 2 {
            break 'search;
        }
    }
}

static mut GLOBAL_COUNT: i32 = 10;

fn next_line() -> Option<String> {
    unsafe {
        GLOBAL_COUNT -= 1;
        if GLOBAL_COUNT <= 0 {
            None
        } else {
            let s = format!("{}", GLOBAL_COUNT);
            Some(s)
        }
    }
}

fn get_some() -> Option<i32> {
    Some(10)
}
