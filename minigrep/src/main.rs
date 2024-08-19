use std::env;

fn main() {
    println!("Rust example of grep!");
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for: {query}");
    println!("In file: {file_path}");
}
