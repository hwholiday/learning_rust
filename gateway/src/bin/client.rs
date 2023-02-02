use bytes::{BufMut, BytesMut};
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() {
    println!("gateway client");
    let mut socket = TcpStream::connect("0.0.0.0:8081").await.unwrap();
    let mut buffer = BytesMut::new();
    let req = String::from("123");
    buffer.put_slice(&req.len().to_be_bytes());
    buffer.put(req.as_bytes());
    println!("{:#?}",&buffer.len());
    socket.write_all(&buffer).await.unwrap();
}