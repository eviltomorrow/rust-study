fn main() {
    println!("Hello, world!");

    let flag = true;
    let c = { if flag { 10 } else { 20 } };
    println!("{}", c);

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

    let mut n = 10;
    while n > 0 {
        print!("{},", n);
        n = n - 1;
    }

    let mut n = 10;
    loop {
        if n < 0 {
            break;
        }
        n = n - 1;
    }

    println!();
    for i in 0..20 {
        print!("{} ", i);
    }
    println!();

    let strings: Vec<String> = vec!["A".to_string(), "B".to_string()];
    for rs in &strings {
        println!("String {:?} is as address: {:p}", *rs, rs);
    }

    let mut strings = vec!["A".to_string(), "B".to_string()];
    for rs in &mut strings {
        rs.push('!');
    }
    for rs in &strings {
        println!("{}", rs);
    }

    let answer = loop {
        if let Some(line) = next_line() {
            if line.starts_with("A") {
                break line;
            }
        } else {
            break "B";
        }
    };
    println!("{}", answer);

    'search: for i in 0..20 {
        print!("{}", i);
        loop {
            if i == 10 {
                break 'search;
            } else {
                break;
            }
        }
    }

    println!();
    let num = 'outer: loop {
        let mut n = 10;
        for i in 1.. {
            if i >= 10 {
                n = n * 2;
                break 'outer n;
            }
        }
    };
    println!("{}", num);

    f();
    f1();

    wait_for_process1(&Process {});
    wait_for_process2(&Process {});

    return_1(true);
    break_1(true);

    let mut v1 = Vec::with_capacity(10);
    v1.push(10);
    v1.push(20);
    println!("{:?}", v1);

    let mut v2 = Vec::<i32>::with_capacity(10);
    v2.push(10);
    v2.push(20);
    println!("{:?}", v2);

    let ramp = (0..10).collect::<Vec<i32>>();
    println!("{:?}", ramp);
}

fn get_code() -> i32 {
    10
}

fn next_line() -> Option<&'static str> {
    Some("A")
}

fn f() {
    return;
}

fn f1() -> () {
    return;
}

#[allow(while_true, unreachable_code)]
fn wait_for_process1(process: &Process) -> i32 {
    while true {
        if process.wait() {
            return process.exit_code();
        }
    }
    0
}

fn wait_for_process2(process: &Process) -> i32 {
    loop {
        if process.wait() {
            return process.exit_code();
        }
    }
}

fn return_1(flag: bool) -> i32 {
    if flag {
        return 10;
    }
    10
}

fn break_1(flag: bool) -> i32 {
    let _ = loop {
        if flag {
            break 10;
        }
    };
    10
}

#[allow(dead_code)]
fn loop_1() -> i32 {
    let mut n = 0;
    loop {
        n = n + 1;
        if n > 10 {
            break 10;
        }
    }
}

struct Process {}

impl Process {
    fn wait(&self) -> bool {
        true
    }

    fn exit_code(&self) -> i32 {
        0
    }
}
