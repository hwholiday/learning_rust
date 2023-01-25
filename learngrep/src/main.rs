use learngrep::Config;
use std::{env, process};

// eprintln 输出到标准错误
// println 输出到标准输出
// cargo run -- body ./test.txt 
fn main() {
    println!("learn grep");
    println!("{}", env::var("PATH").is_ok());
    let flags: Vec<String> = env::args().collect();
    let conf = Config::new(&flags).unwrap_or_else(|err| {
        eprintln!("Config {err}");
        process::exit(1);
    });
    println!("conf {} {}", conf.query, conf.file_path);
    learngrep::run(&conf).unwrap_or_else(|err| {
        eprintln!("run {err}");
        process::exit(1);
    });
}
