use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn print_arg(&self) {
        println!("query = {}\nfile_path = {}", self.query, self.file_path);
    }

    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap();
    config.print_arg();

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("{}", contents);
}
