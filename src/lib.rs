use std::{env, error::Error, fs, vec};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;
    let result = match config.ignore_case {
        true => search_case_insensitive(&config.query, &content),
        false => search_case_sensitive(&config.query, &content),
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    let query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::search_case_insensitive;
    use super::search_case_sensitive;
    use std::vec;

    #[test]
    fn case_sensitive() {
        let query = "cure";
        let content = "\
Rust is a
safe, fast, secure
programming language";
        assert_eq!(
            vec!["safe, fast, secure"],
            search_case_sensitive(query, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust is a
safe, fast, secure,
programming language,
TRUST me";
        assert_eq!(
            vec!["Rust is a", "TRUST me"],
            search_case_insensitive(query, content)
        );
    }
}
