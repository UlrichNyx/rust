//! ### MiniGrep Main
//! Author: UlrichNyx  
//! Date: 2024-11-19  
//! 
//! The entry point for the MiniGrep application.  
//! Handles argument parsing, configuration setup, and error management.  

use std::env;
use std::process;
use minigrep::Config;

/// ### Main Function
/// The entry point of the application.
/// 
/// - Collects command-line arguments.
/// - Builds the `Config` object.
/// - Runs the application logic and handles potential errors.
///
/// ### Behavior:
/// - Exits with code `1` if there is an error parsing arguments or during execution.
fn main() {
    // Parse configuration
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Execute application logic
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
