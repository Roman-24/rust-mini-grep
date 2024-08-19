use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}

fn main() {
    println!("Rust example of grep!");
    let args: Vec<String> = env::args().collect();

    let config = parse_args(&args);

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("Something went wrong reading the file");

    print!("Text:");
    print!("{contents}");
}

fn parse_args(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    return Config { query, file_path };
}
