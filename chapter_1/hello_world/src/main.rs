// ### Author
// 
// UlrichNyx
//
// ### Date
//
// * 2024-10-10
//
// ### Description
//
// A simple Rust program that demonstrates reading user input and displaying a personalized message 
// based on the input provided. It serves as a basic example of using standard input and output in Rust.

use std::io;

/// ### Main function
///
/// This is the entry point of the application. It asks for the user's name, reads it from the standard input,
/// trims any whitespace, and then greets the user with a welcome message.
///
/// ### Panics
///
/// This function will panic if it fails to read a line from the standard input.
fn main() {
    println!("What is your name?");
    
    let mut name: String = String::new();
    
    io::stdin().read_line(&mut name).expect("Failed to read line!");
    
    let name: &str = name.trim();
    
    println!("Hello {}, welcome to the world of Rust!", name);
}
