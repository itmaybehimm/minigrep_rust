use std::env::{self};
use std::error::Error;
use std::fs;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();

    // dbg!(&args);

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Other way
    // let contents: String = match fs::read_to_string(file_path) {
    //     Ok(s) => s,
    //     Err(e) => panic!("{}", e.to_string()),
    // };
    // dbg!(contents);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
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

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.file_path)?;
    println!("{contents}");
    Ok(())
}
