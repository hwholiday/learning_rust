use std::sync::mpsc;
use std::thread;

pub fn th() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || tx.send("111111").unwrap());
    for v in rx {
        println!("{}", v)
    }
}
