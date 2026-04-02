use std::{fs::File, io::Write};

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

fn main() -> GenericResult<()> {
    println!("Hello, world!");

    let mut local_file = File::create("hello.txt")?;
    say_hello1(&mut local_file)?;

    let mut bytes = vec![];
    say_hello2(&mut bytes)?;
    println!("{:?}", String::from_utf8(bytes));

    Ok(())
}

fn say_hello1(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(br"hello world\n")?;
    out.flush()
}

fn say_hello2<T: Write>(out: &mut T) -> std::io::Result<()> {
    out.write_all(br"hello world\n")?;
    out.flush()
}
