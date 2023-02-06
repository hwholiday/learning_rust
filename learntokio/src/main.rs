use core::time;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::task::Waker;
use std::thread;
use std::{pin::Pin, task::Context, task::Poll};
use tokio::signal;
use tokio::sync::Notify;

#[tokio::main]
async fn main() {
    println!("learn tokio");
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();
    tokio::spawn(async move {
        let t = TestFuture::new();
        let result = t.await;
        println!("result {:?}", result);
        notify2.notify_one();
        
    });
    notify.notified().await;
    println!("result1111 notified");
    match signal::ctrl_c().await {
        Ok(()) => {
            println!("service stop succeed")
        }
        Err(err) => {
            eprintln!("service stop: {} failed", err);
        }
    }
}
#[derive(Clone, Debug)]
pub struct TestFuture {
    inner: i64,
    waker: Option<Arc<Mutex<Waker>>>,
}
impl TestFuture {
    fn new() -> TestFuture {
        TestFuture {
            inner: chrono::Local::now().timestamp(),
            waker: None,
        }
    }
}

impl Future for TestFuture {
    type Output = Option<String>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(wk) = &self.waker {
            let mut waker = wk.lock().unwrap();
            if !waker.will_wake(cx.waker()) {
                *waker = cx.waker().clone();
            }
        } else {
            let wk = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(wk.clone());
            thread::spawn(move || {
                thread::sleep(time::Duration::from_secs(10));
                let waker = wk.lock().unwrap();
                waker.wake_by_ref();
            });
        }
        if chrono::Local::now().timestamp() - 5 > self.inner {
            Poll::Ready(Some(chrono::Local::now().to_string()))
        } else {
            Poll::Pending
        }
    }
}
