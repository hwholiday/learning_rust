use std::{collections::HashMap, sync::Arc};

use bytes::BytesMut;
use tokio::{sync::{mpsc, Mutex}, net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};
use uuid::Uuid;

pub type Tx = mpsc::UnboundedSender<String>;

pub type Rx = mpsc::UnboundedReceiver<String>;

pub struct Shared {
    pub peers: HashMap<String, Tx>,
}

impl Shared {
    pub fn new() -> Self {
        Shared {
            peers: HashMap::new(),
        }
    }

    pub async fn single(&mut self, sender: String, message: &str) {
        for peer in self.peers.iter_mut() {
            if *peer.0 == sender {
                let _ = peer.1.send(message.into());
            }
        }
    }
    pub async fn broadcast_all(&mut self, message: &str) {
        for peer in self.peers.iter_mut() {
                let _ = peer.1.send(message.into());
        }
    }
}

pub struct FormatTcp {
    pub stream: TcpStream,
}

impl FormatTcp {
    pub fn new(socket: TcpStream) -> FormatTcp {
        FormatTcp { stream: socket }
    }
    pub async fn read(&mut self) -> Result<Option<BytesMut>, String> {
        let mut dst = [0u8; 8];
        match self.stream.read_exact(&mut dst).await {
            Ok(n) => {
                if n == 0 {
                    return Ok(None);
                }
            }
            Err(e) => return Err(e.to_string()),
        };
        let len = u64::from_be_bytes(dst);
        let mut buffer = BytesMut::with_capacity(len as usize);
        self.stream
            .read_buf(&mut buffer)
            .await
            .or(Err("read_exact failed".to_string()))?;
        Ok(Some(buffer.clone()))
    }
    pub async fn write(&mut self,out:&str) {
        let resp = out.as_bytes();
        let _ = self.stream.write(resp).await;
    }
}

/// The state for each connected client.
pub struct Peer {
    pub format_tcp: FormatTcp,
    pub id: String,
    pub rx: Rx,
}

impl Peer {
    pub async fn new(state: Arc<Mutex<Shared>>, format_tcp: FormatTcp) -> Peer {
        let uuid = Uuid::new_v4();
        let (tx, rx) = mpsc::unbounded_channel();
        state.lock().await.peers.insert(uuid.to_string(), tx);
        Peer {
            id: uuid.to_string(),
            format_tcp: format_tcp,
            rx: rx,
        }
    }
}

impl ToString for Peer {
    fn to_string(&self) -> String {
        format!("[id:{}]", self.id,)
    }
}

pub struct Messages {
    msg: BytesMut,
    create_time: chrono::NaiveDateTime,
}

impl Messages {
    pub fn new() -> Messages {
        Messages {
            msg: BytesMut::with_capacity(1024),
            create_time: chrono::Local::now().naive_local(),
        }
    }
    pub fn set_msg(&mut self, buf: BytesMut) {
        self.msg = buf
    }
}

impl ToString for Messages {
    fn to_string(&self) -> String {
        let buffer = self.msg.to_vec();
        let input = String::from_utf8(buffer).unwrap_or_default();
        format!(
            "[create_time:{} data:{}]",
            self.create_time.format("%Y-%m-%d %H:%M:%S"),
            input.to_string()
        )
    }
}