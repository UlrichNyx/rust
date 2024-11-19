//! ### Rust Search Tool
//! Author: UlrichNyx  
//! Date: 2024-11-19  
//! 
//! A tool to search for specific query strings in a file's content, with support for case-sensitive 
//! and case-insensitive matching.  
//! 
//! This implementation demonstrates parsing command-line arguments, reading files, 
//! and writing modular, testable Rust code.  

use std::fs;
use std::error::Error;
use std::env;

/// ### Config
/// Holds the configuration for the search operation, including:
/// - `query`: The search query.
/// - `filename`: The name of the file to search.
/// - `ignore_case`: Whether the search is case-insensitive.
pub struct Config {
    query: String,
    filename: String,
    ignore_case: bool,
}

impl Config {
    /// Builds a `Config` instance from command-line arguments.
    ///
    /// ### Parameters:
    /// - `args`: A slice of `String` representing the command-line arguments.
    ///
    /// ### Returns:
    /// - `Ok(Config)` on success.
    /// - `Err(&'static str)` if there are insufficient arguments.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let ignore_case = if let Some(_) = args.get(3) {
            match args[3].clone().parse::<bool>() {
                Ok(value) => value,
                Err(_) => false,
            }
        } else {
            env::var("IGNORE_CASE").is_ok()
        };
        println!("{ignore_case}");
        Ok(Config { query, filename, ignore_case })
    }
}

/// Executes the search operation based on the given `Config`.
///
/// ### Parameters:
/// - `config`: The `Config` instance containing query details.
///
/// ### Returns:
/// - `Ok(())` on success.
/// - `Err(Box<dyn Error>)` if an error occurs.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let result = if config.ignore_case {
        case_insensitive_search(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in result {
        println!("{line}");
    }
    Ok(())
}

/// Performs a case-sensitive search.
///
/// ### Parameters:
/// - `query`: The search query.
/// - `contents`: The file contents.
///
/// ### Returns:
/// - A `Vec<&str>` containing lines that match the query.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

/// Performs a case-insensitive search.
///
/// ### Parameters:
/// - `query`: The search query.
/// - `contents`: The file contents.
///
/// ### Returns:
/// - A `Vec<&str>` containing lines that match the query (case-insensitively).
pub fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests case-sensitive search functionality.
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    /// Tests case-insensitive search functionality.
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], case_insensitive_search(&query, &contents));
    }
}
