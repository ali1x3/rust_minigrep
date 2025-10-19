use std::{env, error::Error, fs, process};
use minigrep::search;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enought Arguements");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }

    fn print_args(self) -> Self {
        println!("Searching for {} in {}", self.query, self.file_path);
        self
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args)
        .map(|s| s.print_args()).unwrap_or_else(|_err| {
            println!("Error with parsing");
            process::exit(1);
        });

    let _ = run(config);
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}
