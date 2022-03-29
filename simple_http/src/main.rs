mod client;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

static ADDR:&str ="0.0.0.0:8089";

fn main() {
   let listener = TcpListener::bind(ADDR).unwrap();
    for c in listener.incoming(){
        let mut stream =c.unwrap();
        thread::spawn(move ||
            handle_connection(stream));
    }
    drop(listener)
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1];
    loop{
        stream.read(&mut buffer).expect("read err");
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
        stream.write("server".as_bytes()).unwrap();
        stream.flush().unwrap();
        println!("send success")
    }
}
