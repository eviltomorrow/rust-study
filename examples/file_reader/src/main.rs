use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let path = "Cargo.toml";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}
