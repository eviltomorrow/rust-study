use std::fs::{self, File};

fn main() {
    /*
     * rust 错误分为两大类：可恢复的（recoverable）和不可恢复的（unrecoverable）
     * 1、不可恢复错误
     *       显示调用 panic! 宏
     *       访问数组下标越界这种运行时错误
     */
    // panic!("there is one panic");

    let v = vec![1, 2, 3];
    let a3 = v[2];
    // let a99 = v[99];
    println!("{a3}");

    let greeting_file_result = File::open("hello.txt");
    match greeting_file_result {
        Ok(_) => println!("ok"),
        Err(err) => println!("{:#?}", err),
    }

    let read_text = fs::read_to_string("hello.txt").unwrap();
    println!("{}", read_text);
}
