use gateway::{pkg::conn::Conn, setup};
use tokio::net::{TcpListener, TcpStream};
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
        info!("socket_addr {:?}", socket_addr);
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    let mut conn = Conn::new(socket);
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
    loop {
        tokio::select! {
            req = conn.read_cmd()=>{
                match req {
                    Ok(Some(v)) => {
                        println!("v {}", v);
                    },
                    Ok(None) => {
                        println!("None");
        
                    },
                    Err(e) => {
                        println!("e {}", e);
                    }
                }
            },

            _ = interval.tick() => {
                println!("tick tick tick");
            }
        }
        
    }
}
