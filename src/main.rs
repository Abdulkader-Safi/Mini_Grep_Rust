use std::env;
use std::process;

use mini_grep::run;
use mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    eprintln!("Searching for {}", config.query);
    eprintln!("In file {}", config.file_name);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
