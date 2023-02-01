use gateway::setup;
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info};

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

async fn process(socket: TcpStream) {}
