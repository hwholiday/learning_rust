use mini_redis::client;

#[tokio::main]
async fn main() {
    println!("learn tokio client");
    let (tx, mut rx) = mpsc::channel(20);
    let tx1 = tx.clone();
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };
        tx.send(cmd).await.unwrap();
        let resp = resp_rx.await;
        println!("t1 resp {:?}", resp);
    });

    let t2 = tokio::spawn(async move {
        let (req, res) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: req,
        };
        tx1.send(cmd).await.unwrap();
        let resp = res.await;
        println!("t2 resp {:?}", resp);
    });
    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:8081").await.unwrap();
        while let Some(val) = rx.recv().await {
            match val {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    let _ = resp.send(res);
                }
            }
        }
    });
    t2.await.unwrap();
    t1.await.unwrap();
    manager.await.unwrap();
}
use bytes::Bytes;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

type Sender<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Sender<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Sender<()>,
    },
}
