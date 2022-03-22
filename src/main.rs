use std::{env, process};
use minigrep::{Config,run};
fn main() {
    //cargo run -- nobody ./minigrep/2.txt
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Config::new err {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}

