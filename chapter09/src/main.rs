use std::fs::File;

fn main() {
    println!("Hello, world!");
    // panic!("crash");
    let greeting_file_result = File::open("note.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {}", error),
    };
}
