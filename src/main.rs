use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("m̀inigrep: problem parsing arguments {err}");
        process::exit(1)
    });

    println!(
        "Searching for {}\nInside {}",
        config.query, config.file_path
    );

    if let Err(e) = minigrep::run(config) {
        println!("minigrep: {e}");
    }
}
