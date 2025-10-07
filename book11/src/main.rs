use std::io::Write;

type GenericError = Box<dyn std::error::Error>;

fn main() -> Result<(), GenericError> {
    let mut bytes = vec![];
    let r = say_hello(&mut bytes);
    match r {
        Ok(_) => println!("ok"),
        Err(err) => println!("{}", err),
    }
    Ok(())
}

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\r\n")?;
    Ok(())
}
