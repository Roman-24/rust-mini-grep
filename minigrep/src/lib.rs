use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub is_case_sensitive: bool,
}

impl Config {
    pub fn parse_args(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from("Not enough arguments"));
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let is_case_sensitive = env::var("IS_CASE_SENSITIVE").is_ok();

        return Ok(Config {
            query,
            file_path,
            is_case_sensitive,
        });
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    println!("Searching w case sensitive.");
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    println!("Searching w case insensitive.");
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let search_result = if config.is_case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    println!("Results:");
    for line in search_result {
        println!("{}", line);
    }

    Ok(())
}
