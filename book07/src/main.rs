use std::{thread, time};

fn main() {
    println!("Hello, world!");

    let handler = thread::spawn(|| {
        let c = [0_i32; 2];
        for i in 0..100 {
            let c = c[i];
            println!("{}", c);
            thread::sleep(time::Duration::from_secs(1));
        }
    });
    handler.join().expect("Boom!");
}
