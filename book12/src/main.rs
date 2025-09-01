use std::{
    fs::OpenOptions,
    io::{self, Write},
};

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let path = String::from("/tmp/hello.txt");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("open file failure");

    for line in vec!["你好-1", "你好-2", "你好-3"] {
        writeln!(file, "{}", line)?;
    }

    file.flush()?;
    Ok(())
}
