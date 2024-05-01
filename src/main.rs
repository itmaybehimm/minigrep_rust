use std::env::{self};
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();

    // dbg!(&args);

    let config = Config::new(&args);

    // Other way
    // let contents: String = match fs::read_to_string(file_path) {
    //     Ok(s) => s,
    //     Err(e) => panic!("{}", e.to_string()),
    // };
    // dbg!(contents);

    let contents: String =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}
