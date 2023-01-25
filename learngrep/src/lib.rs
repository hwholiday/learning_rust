use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;

pub fn run(conf: &Config) -> Result<(), Box<dyn Error>> {
    let contents = read_file_by_path(&conf.file_path)?;
    for result in search(&conf.query, &contents) {
        println!("{}", result);
    }
    Ok(())
}

fn read_file_by_path(path: &str) -> Result<String, io::Error> {
    let mut result = String::new();
    File::open(path)?.read_to_string(&mut result)?;
    Ok(result)
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(flags: &[String]) -> Result<Config, &'static str> {
        if flags.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config {
            query: flags[1].clone(),
            file_path: flags[2].clone(),
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
