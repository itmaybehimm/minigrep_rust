use std::env;
use std::error::Error;
use std::fs;
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query: String = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();
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

    #[test]
    fn case_sensitive() {
        let query_1: &str = "duct";
        let query_2: &str = "Duct";
        let query_3: &str = "aFe";
        let result_3: Vec<&str> = Vec::new();
        let contents: &str = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query_1, contents));
        assert_eq!(vec!["Duct tape."], search(query_2, contents));
        assert_eq!(result_3, search(query_3, contents));
    }

    #[test]
    fn case_insensitive() {
        let query_1: &str = "rUsT";
        let query_2: &str = "Duct";
        let query_3: &str = "aFe";

        let contents: &str = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
        assert_eq!(vec!["Rust:"], search_case_insensitive(query_1, contents));
        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape."],
            search_case_insensitive(query_2, contents)
        );
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensitive(query_3, contents)
        );
    }
}
