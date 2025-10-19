use std::{env, error::Error, fs, process};
use minigrep::{case_insensitive_search, search};

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enought Arguements");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
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
            eprintln!("Error with parsing");
            process::exit(1);
        });

    if let Err(e) = run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        case_insensitive_search(&config.query, &contents)
    }
    else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}
