use std::env;
use std::fs;

fn main() {
    println!("Rust example of grep!");
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_args(&args);

    println!("Searching for: {query}");
    println!("In file: {file_path}");

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    print!("Text:");
    print!("{contents}");
}

fn parse_args(args: &[String]) -> (String, String) {
    let query = &args[1];
    let file_path = &args[2];
    (query.to_string(), file_path.to_string())
}
