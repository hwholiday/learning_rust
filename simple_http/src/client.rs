use std::{io, thread, time};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread::Thread;
use std::time::Duration;


#[cfg(test)]
mod tests {
    #[test]
    fn test_client() {
        super::client()
    }
}

#[allow(dead_code)]
fn client() {
    let mut client = TcpStream::connect(super::ADDR).unwrap();
    let mut buffer = [0; 6];
    for i in 1..10{
        client.write(i.to_string().as_bytes()).unwrap();
        client.flush().unwrap();
        client.read(&mut buffer).expect("read err");
        println!("res: {}", String::from_utf8_lossy(&buffer[..]));
       // client.read_to_string(&mut output).unwrap();
    }

}
