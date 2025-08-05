fn main() {
    println!("Hello, world!");

    let mut x = 10;
    loop {
        let d = 100 / x;
        println!("{}", d);
        x -= 1;
    }
}
