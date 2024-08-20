use std::env;
use std::process;

use minigrep::Config;

fn main() {
    println!("Rust example of grep!");
    let args: Vec<String> = env::args().collect();

    let config = Config::parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
