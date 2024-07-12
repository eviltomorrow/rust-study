fn main() {
    // 3.1
    println!("Hello, world!");
    let x = 10;
    println!("The value is: {x}");

    let mut y = 5;
    println!("The value of y is: {y}");

    y = 20;
    println!("The value of y is: {y}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");

    let x = 20;
    println!("The vlaue is: {x}");
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value is: {x}");
    }
    println!("The value is: {x}");

    let spaces = "      ";
    let spaces = spaces.len();
    println!("The lenght of spaces is: {spaces}");

    // 3.2
    let guess: f64 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {guess}");

    let a: u32 = 100_00;
    let b: i32 = -100_00_0;
    println!("The value of a, b is {a}, {b}");

    let a1: u32 = 98_222;
    let b1: u32 = 0xff;
    let c1: u32 = 0o77;
    let d1: u32 = 0b1111_0000;
    let e1: u8 = b'A';
    let f1: u8 = 121;
    println!("{a1}, {b1}, {c1}, {d1}, {e1}, {f1}");

    let a2: f32 = 2.0;
    let b2: f64 = 4.0;
    println!("{a2}, {b2}");

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;
    println!("{sum}, {difference}, {product}, {quotient}, {truncated}, {remainder}");

    let a3: bool = true;
    println!("{a3}");

    let a4: char = 'c';
    let b4 = 'd';
    println!("{a4}, {b4}");

    let tup: (i32, f64, i8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{x}, {y}, {z}");
    println!("{}, {}", tup.0, tup.1);

    let a5 = [1, 2, 3, 4, 5];
    _ = a5;
    let b5: [i32; 5] = [1, 2, 3, 4, 5];
    _ = b5;
    let c5 = [3; 5];
    _ = c5;
    println!("{},{},{},{},{}", c5[0], c5[1], c5[2], c5[3], c5[4]);

    another_function();
    another_function_1(10);
    print_labeled_measurement(10, 'c');

    let y = {
        let x = 1;
        x + 2
    };
    println!("{y}");

    println!("{}", five());

    let t = ();
    _ = t;

    plus_one(10);

    let x = 5;
    if x < 10 {
        println!("x < 10");
    } else {
        println!("x >= 10");
    }

    let x = 10;
    if x == 3 {
        println!("x == 3");
    } else if x == 4 {
        println!("x == 4");
    } else {
        println!("unknown!");
    }

    let condition = true;
    let number = if condition { 10 } else { 20 };
    println!("{}", number);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };
    println!("{result}");

    let mut counter = 0;
    'counting_up: loop {
        println!("count: {}", counter);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    println!("End count = {}", counter);

    let mut counter = 3;
    while counter != 0 {
        println!("{counter}");
        counter -= 1;
    }
    println!("over");

    let a = [10, 20, 30, 40, 50];
    for e in a {
        println!("{}", e);
    }

    for number in (1..4).rev() {
        println!("number is {}", number);
    }
}

fn another_function() {
    println!("Another function.");
}

fn another_function_1(x: i32) {
    println!("Another function 1: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}, {}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
