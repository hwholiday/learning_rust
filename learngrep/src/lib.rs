use std::env;
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
    pub fn new(mut flags: env::Args) -> Result<Config, &'static str> {
        if flags.len() < 3 {
            return Err("not enough arguments");
        }
        flags.next();
        let query =  match flags.next(){
            Some(v) =>v,
            None=>  return Err("query is empty"),
        };
        let file_path =  match flags.next(){
            Some(v) =>v,
            None=>  return Err("file_path is empty"),
        };
        Ok(Config {
            query: query,
            file_path: file_path,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
    //  let mut result = Vec::new();
    // for line in contents.lines() {
    //    if line.contains(query) {
    //       result.push(line)
    //  }
    // }
    // result
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
