use gateway::setup;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};
use tracing::info;

#[tokio::main]
async fn main() {
    setup();
    info!("gateway");
    service().await;
}

async fn service() {
    let listener = TcpListener::bind("0.0.0.0:8081").await.unwrap();
    loop {
        let (socket, socket_addr) = listener.accept().await.unwrap();
        println!("socket_addr {:?}", socket_addr);
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(mut socket: TcpStream) {
    let mut dst = [0u8; 8];
    socket.read_exact(&mut dst).await.unwrap();
    println!("1 {:?}", dst);
    let len = u64::from_be_bytes(dst);
    println!("2 {:?}", len);
    let mut buffer = vec![0u8; len as usize];
    socket.read_exact(&mut buffer).await.unwrap();
    println!("3 {:?}", &buffer);
    println!("4 {:?}", String::from_utf8(buffer).unwrap());
}
