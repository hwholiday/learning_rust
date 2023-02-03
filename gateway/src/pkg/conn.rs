use std::{io};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct Conn {
    id: String,
    stream: TcpStream,
}

impl Conn {
    pub fn new(socket: TcpStream) -> Conn {
        let uuid = Uuid::new_v4();
        Conn {
            id: uuid.to_string(),
            stream: socket,
        }
    }
    
    pub async fn read_cmd(&mut self) -> Result<Option<String>, String> {
        let mut dst = [0u8; 8];
        match self.stream.read_exact(&mut dst).await {
            Ok(n) => {
                println!("input read_exact n {:?}", n);
                if n == 0 {
                    //peer socket is dead
                   let _ = self.stream.shutdown().await;
                    return Err("connection reset by peer".to_string())
                }
            }
            Err(e) => {
                return Err(e.to_string())
            }
        };
        let len = u64::from_be_bytes(dst);
        let mut buffer = vec![0u8; len as usize];
        self.stream.read_exact(&mut buffer).await.or(Err("read_exact failed".to_string()))?;
        let  mut input = String::new();
        match String::from_utf8(buffer){
            Ok(v) => {
                input.push_str(&v.to_string())
            },
            Err(e)=>{
                return Err(e.to_string())
            }
        }
        println!("input {:?}", &input.trim());
        println!("out {:?}", format!("{}out", &input.trim()));
        Ok(Some(input.clone().trim().to_string()))
    }
}

impl ToString for Conn {
    fn to_string(&self) -> String {
        format!("[id:{}]", self.id,)
    }
}
