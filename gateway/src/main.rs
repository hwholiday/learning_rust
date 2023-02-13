use std::sync::Arc;

use gateway::pkg::conn::{Shared, FormatTcp, Peer, Messages};
use gateway::setup;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex};

use tracing::info;
#[tokio::main]
async fn main() {
    setup();
    info!("gateway service");
    service().await;
}

async fn service() {
    let listener = TcpListener::bind("0.0.0.0:8081").await.unwrap();
    let state = Arc::new(Mutex::new(Shared::new()));
    loop {
        let (socket, socket_addr) = listener.accept().await.unwrap();
        info!("socket_addr {:?}", socket_addr);
        let s = Arc::clone(&state);
        tokio::spawn(async move {
            process(s, socket).await;
        });
    }
}

async fn process(state: Arc<Mutex<Shared>>, socket: TcpStream) {
    let mut conn = Peer::new(state.clone(), FormatTcp::new(socket)).await;
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
    loop {
        tokio::select! {
            req = conn.format_tcp.read()=>{
                match req {
                    Ok(Some(buf)) => {
                        let mut msg = Messages::new();
                        msg.set_msg(buf);
                        let mut state = state.lock().await;
                        println!("conn read {}",&msg.to_string());
                        state.broadcast_all(&msg.to_string()).await;
                    },
                    Ok(None) => {
                        println!("conn is close");
                    },
                    Err(e) => {
                        println!("conn {} failed", e);
                        return;
                    }
                }
            },
            _ = interval.tick() => {
                println!("tick");
            },
            Some(msg) = conn.rx.recv()=>{
                println!("msg =>  {}",&msg);
                conn.format_tcp.write(&msg.to_string()).await;
                
            }
        }
    }
}

