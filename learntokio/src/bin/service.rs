use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use bytes::Bytes;



#[tokio::main]
async fn main() {
    println!("learn tokio service");
    service().await;
}
type Db = Arc<Mutex<HashMap<String, Bytes>>>;
async fn service() {
    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, socket_addr) = listener.accept().await.unwrap();
        println!("socket_addr {:?}", socket_addr);
        let db = db.clone();
        tokio::spawn(async move {
            process(socket,db).await;
        });
    }
}

async fn process(socket: TcpStream,db:Db) {
    let mut connection = Connection::new(socket);
    while  let Some(frame) = connection.read_frame().await.unwrap() {
        // Respond with an error
        println!("process frame {:?}",frame);
        let req = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(val) = db.get(cmd.key()) {
                    Frame::Bulk(val.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&req).await.unwrap();
    }
}
