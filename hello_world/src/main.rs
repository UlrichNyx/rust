
use std::io;
fn main() {
    println!("What is your name?");
    let mut name : String = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line!");
    let name: &str = name.trim();
    println!("Hello {}, welcome to the world of Rust!", name);
}

/* Standard solution

fn main() {
    println!("Hello, world!");
}

 */