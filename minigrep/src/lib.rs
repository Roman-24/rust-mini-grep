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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    print!("Text:");
    print!("{contents}");

    Ok(())
}
