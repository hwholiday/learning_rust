use mini_redis::{client, Result};
use mini_redis::{Connection, Frame};
use tokio::{
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    println!("learn tokio");
    service().await;
}

async fn service() {
    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    loop {
        let (socket, socket_addr) = listener.accept().await.unwrap();
        println!("socket_addr {:?}", socket_addr);
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);
    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);
        // Respond with an error
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}


async fn redis() -> Result<()> {
    let mut client = client::connect("127.0.0.1:8081").await?;
    client.set("hw", "test-hw".into()).await?;
    let val = client.get("hw").await?;
    println!("result={:?}", val);
    Ok(())
}


#[cfg(test)]
mod tests{
    use super::*;

    #[tokio::test]
     async fn test_fn_redis(){
        redis().await.unwrap();
    }
}