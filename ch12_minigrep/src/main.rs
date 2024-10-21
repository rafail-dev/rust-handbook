use std::env;
use std::process;

use ch12_minigrep::Config;

// $ IGNORE_CASE=1 cargo run -- to poem.txt

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(err) = ch12_minigrep::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
