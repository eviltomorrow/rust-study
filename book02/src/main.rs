use std::{thread, time::Duration};

fn main() {
    /*
    - 语言规范
    - 编译器
    - 核心库
    - 标准库
    - 包管理器
    */

    println!("Hello world");

    let handler = thread::spawn(|| {
        let mut i = 0;
        loop {
            i += 1;
            println!("Hello world, {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    handler.join().unwrap();
}
