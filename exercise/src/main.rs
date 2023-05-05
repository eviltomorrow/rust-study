fn main() {
    println!("Hello, world!");
    let mut x = 20;
    x = x + 20;
    println!("x is {x}");

    say();
    say_something("shepard");
    println!("return {}", return_something());
    println!("{}", THREE_HOURS_SECONDS);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

    let a = 10.0;
    let b: f64 = 20.0;
    let sum = a + b;
    println!("sum is {}", sum);
    let sum = a * b;
    println!("sum is {}", sum);

    let c: f64 = 30.0;
    println!("sum is {}", a + c);

    let t: bool = false;
    if !t {
        println!("t is {}", t);
    }

    let mut c = 'a';
    println!("c is {c}");
    c = 'b';
    println!("c is {}", c);
    c = '你';
    println!("c is {}", c);

    let tup = (3, 2.0, 'c', true);
    println!("tup is {:#?}", tup);

    let a = [1, 2, 3];
    println!("{:?}", a);
    println!("{:?}", a);

    for e in a {
        println!("e is {:?}", e);
    }

    let mut count = 0;
    loop {
        count += 1;
        if count >= 10 {
            break;
        }
        println!("count-{}", count);
    }

    while count > 5 {
        count -= 1;
        println!("count-{count}");
    }
}

const THREE_HOURS_SECONDS: i64 = 32 * 60 * 60;

fn say() {
    println!("hello world");
}

fn say_something(world: &str) {
    println!("hello {}", world);
}

fn return_something() -> String {
    String::from("Hello world")
}
