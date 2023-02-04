use bytes::BytesMut;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
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

    pub async fn close(&mut self) -> io::Result<()> {
        self.stream.shutdown().await
    }
}

impl ToString for Conn {
    fn to_string(&self) -> String {
        format!("[id:{}]", self.id,)
    }
}

#[derive(Clone, Debug)]
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
