use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io::{self, Write};

fn main() {
    print!("Please enter a file name: ");
    io::stdout().flush().expect("Failed to flush");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let filename = input.trim();
    let greeting_file_result = File::open(filename);

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // Ask if they would like to create
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
    // Read the contents of the file
    // Implement retry functionality
}