use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_args(&args);

    println!("Searching for {query} in {file_path}");

    let file_contents = fs::read_to_string(file_path);

    match file_contents {
        Ok(contents) => println!("With text: \n {contents}"),
        Err(e) => println!("Error: {e}"),
    }
}

fn parse_args(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];
    (query, file_path)
}
