use chapter12::Config;
use std::{env, process};

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    println!("{}, {}", config.query, config.file_path);

    if let Err(error) = chapter12::run(config) {
        eprintln!("Application error: {}", error);
        process::exit(1);
    }
}
