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
    println!("{0}, {1}", tup.0, tup.1);
}
