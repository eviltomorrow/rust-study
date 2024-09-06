use std::{sync::mpsc, thread, time::Duration};

fn main() {
    println!("Hello, world!");

    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(100));
        }
    });
    for i in 1..=5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(10))
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("{:?}", v);
    });
    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let val = rx.recv().unwrap();
    println!("val: {}", val);
}
