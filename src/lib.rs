use std::{error::Error, fs, vec};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;
    for line in search(&config.query, &content) {
        println!("{line}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::search;
    use std::vec;

    #[test]
    fn grep_text() {
        let query = "cure";
        let content = "\
Rust is a
safe, fast, secure
programming language";
        assert_eq!(vec!["safe, fast, secure"], search(query, content));
    }
}
