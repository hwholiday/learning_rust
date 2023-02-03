use learntcp::setup;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tracing::info;
#[tokio::main]
async fn main() {
    setup();
    info!("gateway service");
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
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
    loop {
        tokio::select! {
            req = socket.read_exact(&mut dst) =>{
                match req{
                    Ok(n) =>{
                        println!("input read_exact n {:?}", n);
                        if n == 0 {
                            //peer socket is dead
                            socket.shutdown().await.unwrap();
                        } 
                    },
                    Err(e) =>{
                        println!("input read_exact e {:?}", e);
                        return ;
                    }
                };
                let len = u64::from_be_bytes(dst);
                let mut buffer = vec![0u8; len as usize];
                socket.read_exact(&mut buffer).await.unwrap();
                let input = String::from_utf8(buffer).unwrap();
                println!("input {:?}", input.trim());
                println!("out {:?}",format!("{}out", input.trim()));
                socket
                    .write(format!("{}out", input.trim()).as_bytes())
                    .await
                    .unwrap();
            }
            _ = interval.tick() => {
                println!("tick tick tick");
            }
        }
       
    }
}