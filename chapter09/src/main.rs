use std::io::{self, Read};
use std::{fs::File, io::ErrorKind};
use std::fs;

fn main() {
    /*
       rust 错误分为两大类：可恢复的（recoverable）和不可恢复的（unrecoverable）
       1、不可恢复错误
            显示调用 panic! 宏
            访问数组下标越界这种运行时错误
    */
    // panic!("there is one panic");

    let v = vec![1, 2, 3];
    let a3 = v[2];
    // let a99 = v[99];
    println!("{a3}");

    /*
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }

        1、unwrap 方法，如果 Result 值是成员 Ok， unwrap 会返回 Ok 中的值，如果 Result 是成员 Err，会调用 panic!。
        2、expect 方法，同 unwrap 类似，但使用的错误信息将是我们传递给 expect 的参数。

        3、传播错误 return Err(e);
        4、? 运算符只能被用于返回值与 ? 作用的值相兼容的函数。
    */
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file: {:#?}", error),
            },
            other_error => panic!("Problem openning the file: {:#?}", other_error),
        },
    };
    _ = greeting_file;

    let greeting_file =
        File::open("hello.txt").expect(format!("Open file {} failure", "hello.txt").as_str());
    _ = greeting_file;

    let username_result = read_username_from_file();
    let username = username_result.expect("read username from file failure");
    println!("username is {}", username);

    let username_result = read_username_from_file2();
    let username = username_result.expect("read username from file failure");
    println!("username is {}", username);

    let username_result = read_username_from_file3();
    let username = username_result.expect("read username from file failure");
    println!("username is {}", username);

    let username_result = read_username_from_file4();
    let username = username_result.expect("read username from file failure");
    println!("username is {}", username);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let greeting_file_result = File::open("hello.txt");
    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();
    match greeting_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut greeing_file = File::open("hello.txt")?;
    let mut username = String::new();
    greeing_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}