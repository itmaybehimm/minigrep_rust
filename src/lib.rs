use std::error::Error;
use std::fs;
pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "duct";
        let contents: &str = "\
   Rust:
   safe, fast, productive.
   Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
