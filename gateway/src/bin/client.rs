use bytes::{BufMut, BytesMut};
use tracing::info;
use std::io;
use tokio::{
    io::{AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    println!("gateway client");
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
        // 即便readable()返回代表可读，但读取时仍然可能返回WouldBlock
        match read.try_read_buf(&mut buffer) {
            Ok(0) => {
                // 成功读取了n个字节的数据
                info!("read end");
                break;
            }
            Ok(_) => {
                // 成功读取了n个字节的数据
                println!("out: {:?}", String::from_utf8(buffer));
                continue;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                info!("error {}",e);
                break;
            }
        }
    }
}
