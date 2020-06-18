use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub search_item: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let search_item = match args.next() {
            Some(arg) => arg,
            None => return Err("Search term not provided!"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Filename not provided!"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            search_item,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = if config.case_sensitive {
        search(&config.search_item, &contents)
    } else {
        search_insensitive(&config.search_item, &contents)
    };

    if results.is_empty() {
        println!(
            " Word '{}' not found in file '{}'.",
            &config.search_item, &config.filename
        );
    }
    for line in results {
        println!("{}", line)
    }

    Ok(())
}

pub fn search<'a>(search_item: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(search_item))
        .collect()
}

pub fn search_insensitive<'a>(search_item: &str, contents: &'a str) -> Vec<&'a str> {
    let search_item = search_item.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&search_item) {
            results.push(line.trim());
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
            Hello
            what is a Duct tape?
            Production cost is less
            Dreaming of that face again.";
        assert_eq!(vec!["Production cost is less"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "\
            Hello
            what is a Duct tape?
            Production cost is less
            Dreaming of that face again.";
        assert_eq!(
            vec!["what is a Duct tape?", "Production cost is less"],
            search_insensitive(query, contents)
        );
    }
}
