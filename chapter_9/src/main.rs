//! ### File Handling Example
//! Author: UlrichNyx  
//! Date: 2024-11-19  
//! 
//! This program demonstrates basic file operations in Rust, including:
//! - Reading user input for a filename.
//! - Attempting to open the specified file.
//! - Creating the file if it does not exist.
//! - Reading and displaying file contents.
//! - Using `Result` and `match` for error handling.

use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io::{self, Write};

/// ### Main Function
/// The entry point for the program.
///
/// - Prompts the user to input a filename.
/// - Attempts to open the specified file.
/// - Creates the file if it doesn't exist, or handles other errors.
/// - Reads the file's contents and displays them on the console.
///
/// ### Behavior:
/// - On `ErrorKind::NotFound`, the program attempts to create the file.
/// - On other errors, the program terminates with a `panic!`.
fn main() {
    // Prompt the user for a filename
    print!("Please enter a file name: ");
    io::stdout().flush().expect("Failed to flush");

    // Read user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let filename = input.trim();

    // Attempt to open the file
    let greeting_file_result = File::open(filename);

    // Handle potential errors during file operations
    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // If the file is not found, attempt to create it
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            // For other errors, terminate the program
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // Read the file's contents into a buffer and print them
    let mut buffer = String::new();
    greeting_file.read_to_string(&mut buffer).expect("Failed to read the file");
    println!("{}", buffer);
}
