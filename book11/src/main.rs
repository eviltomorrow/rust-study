use std::io::{self, Write};

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

fn main() -> Result<(), GenericError> {
    println!("Hello, world!");

    use std::fs::File;
    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file)?;

    let mut bytes = vec![];
    say_hello(&mut bytes)?;
    println!("{:?}", bytes);

    let s = min(10, 20);
    println!("{}", s);

    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"Hello")?;

    let mut buf = vec![];
    buf.push(10);

    let writer: &mut dyn Write = &mut buf;
    let _ = writer;
    Ok(())
}

fn say_hello(out: &mut dyn io::Write) -> io::Result<()> {
    out.write(b"hello world\n")?;
    Ok(())
}

fn min<T: Ord>(value1: T, value2: T) -> T {
    if value1 < value2 { value1 } else { value2 }
}
