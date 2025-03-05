use std::{env, error::Error, fs};

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
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(s) => s,
            None => return Err("didnt get a query string"),
        };

        let file_path = match args.next() {
            Some(s) => s,
            None => return Err("didnt get a file path "),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|lines| lines.to_lowercase().contains(&query.to_lowercase()))
        .collect()
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
