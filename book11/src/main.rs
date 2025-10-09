use std::{collections::HashMap, fmt::Debug, fs::File, io::Write, vec};

type GenericError = Box<dyn std::error::Error>;

fn main() -> Result<(), GenericError> {
    let mut bytes = vec![];
    let r = say_hello(&mut bytes);
    match r {
        Ok(_) => println!("ok"),
        Err(err) => println!("{}", err),
    }
    println!("{:?}", bytes);

    let mut local_file = File::create("hello.txt")?;
    let _ = say_hello(&mut local_file)?;

    let mut buf = vec![];
    let size = buf.write(b"Hello world")?;
    println!("{:?}, {}", buf, size);

    let mut buf: Vec<u8> = vec![];
    let writer: &mut dyn Write = &mut buf;
    writer.write(b"Hello world!")?;

    let mut w: Box<&mut dyn Write> = Box::new(&mut local_file);
    w.write(b"Hi")?;

    say_hello2(&mut local_file)?;
    say_hello2::<File>(&mut local_file)?;
    say_hello3::<File>(&mut local_file)?;

    top_ten1::<Vec<i32>>(&vec![]);
    top_ten2::<HashMap<String, i32>>(&HashMap::new());

    let v1 = (0..10).collect::<Vec<i128>>();
    println!("{:?}", v1);

    Ok(())
}

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\r\n")?;
    Ok(())
}

fn say_hello2<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world\r\n")?;
    Ok(())
}

fn say_hello3<W>(out: &mut W) -> std::io::Result<()>
where
    W: Write,
{
    out.write_all(b"say_hello3")?;
    Ok(())
}

fn top_ten1<T: Debug + Eq>(_: &T) {}

fn top_ten2<T>(_: &T)
where
    T: Debug,
    T: Eq,
    T: Debug + Eq,
{
}
