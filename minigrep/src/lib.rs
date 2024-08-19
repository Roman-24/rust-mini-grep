use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn parse_args(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from("Not enough arguments"));
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        return Ok(Config { query, file_path });
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let search_result = search(&config.query, &contents);
    println!("Results:");
    for line in search_result {
        println!("{}", line);
    }

    Ok(())
}
