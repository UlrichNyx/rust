use std::fs;
use std::error::Error;
pub struct Config
{
    query: String,
    filename: String
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }   
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {query, filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents)
    {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines(){
        if line.contains(query)
        {
            results.push(line);
        }
    }
    results
}

pub fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}


#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search(&query, &contents));
    }
}