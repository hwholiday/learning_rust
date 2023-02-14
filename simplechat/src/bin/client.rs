use bytes::{BufMut, BytesMut};
use std::io;
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() {
    println!("simplechat client");
    let socket = TcpStream::connect("0.0.0.0:8081").await.unwrap();
    let (read, mut write) = socket.into_split();
    tokio::spawn(async move {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            println!("input: {}", input.trim());
            let mut buffer = BytesMut::new();
            buffer.put_slice(&input.len().to_be_bytes());
            buffer.put(input.as_bytes());
            write.write_all(&buffer).await.unwrap();
        }
    });
    loop {
        // 等待可读事件的发生
        read.readable().await.unwrap();
        let mut buffer = vec![];
        match read.try_read_buf(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                println!("GOT = {} {:?} {:?}", n, String::from_utf8(buffer.clone()),buffer.len());
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                println!("e {:?}", e);
                return;
            }
        }
    }
}
