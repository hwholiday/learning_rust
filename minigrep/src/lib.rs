use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::{env, process};

#[cfg(test)]
mod tests {
    #[test]
    // cargo test mini_grep  -- frog 2.txt  --show-output
    fn mini_grep() {
        super::mini_grep()
    }
}

#[derive(Debug)]
struct Config {
    grep: String,
    name: String,
}

impl Config {
    fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        args.next();
        args.next();
        let grep_str = match args.next() {
            Some(t) => t,
            None => return Err("did's get grep args"),
        };
        let name_str = match args.next() {
            Some(t) => t,
            None => return  Err("did's get file args"),
        };
        Ok(Config {
            grep: grep_str,
            name: name_str,
        })
    }
}

fn mini_grep() {
    let host_name = env::var("HOSTNAME").expect("HOSTNAME not definition");
    println!("HOSTNAME {}", host_name);
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Config::new err {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(&config.name)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    for line in search(&config.grep, &content) {
        println!("line {}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();
}
