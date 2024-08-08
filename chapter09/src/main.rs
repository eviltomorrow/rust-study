use std::fs;
use std::io::{self, Read};
use std::{fs::File, io::ErrorKind};

fn main() {
    println!("Hello, world!");

    let greeting_file_result = File::open("note.txt");
    let mut _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("note.txt") {
                Ok(file) => file,
                Err(error) => panic!("create file failure, nest error: {}", error),
            },
            kind_error => {
                panic!("{:?}", kind_error);
            }
        },
    };

    let _greeting_file = File::open("note.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("note.txt").unwrap_or_else(|error| {
                panic!("create file failure, nest error: {}", error);
            })
        } else {
            panic!("open file failure, nest error: {}", error);
        }
    });

    // let _greeting_file = File::open("note2.txt").unwrap();
    let _greeting_file = File::open("note.txt").expect("open file failure");

    test_func(false);

    match read_username_from_file("note.txt") {
        Ok(data) => println!("the value is: {:?}", data),
        Err(error) => panic!("read_username_from_file failure, nest error: {:?}", error),
    }

    let read_username_from_file2_result = read_username_from_file2("note.txt");
    if let Ok(data) = read_username_from_file2_result {
        println!("data: {:?}", data);
    }

    let result = read_username_from_file3("note.txt");
    if let Ok(data) = result {
        println!("data: {:?}", data);
    }

    let result = read_username_from_file4("note4.txt");
    if let Ok(data) = result {
        println!("data: {:?}", data);
    }

    let result = read_username_from_file4("note.txt");
    let data = if let Ok(data) = result {
        data
    } else {
        String::from("a")
    };
    println!("data: '{}'", data);

    let result = read_username_from_file5("note2.txt");
    if let Err(error) = result {
        println!("{:?}", error);
    }
}

fn test_func(flag: bool) -> String {
    if flag {
        panic!("test_func panic!");
    }
    String::from("Hello world")
}

fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let username_file_result = File::open(path);
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

fn read_username_from_file2(path: &str) -> Result<String, std::io::Error> {
    let mut username_file = File::open(path)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file3(path: &str) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(path)?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file4(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn read_username_from_file5(path: &str) -> Result<String, OutError> {
    let mut username = String::new();
    File::open(path)?.read_to_string(&mut username)?;
    Ok(username)
}

#[derive(Debug)]
struct OutError {}

impl From<io::Error> for OutError {
    fn from(_value: io::Error) -> Self {
        OutError {}
    }
}
