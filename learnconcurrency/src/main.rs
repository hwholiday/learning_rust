use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("learn concurrency");
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for val in 1..10 {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for val in rx {
        println!("rx is : {}", val);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex, RwLock};
    use std::thread;
    #[test]
    fn test_thread() {
        let test = "11111";
        let thread_one = thread::spawn(move || {
            println!("thread 1 {}", test);
        });
        let thread_two = thread::spawn(|| {
            println!("thread 2");
        });
        println!("thread 0 {}", test);
        thread_one.join().unwrap();
        thread_two.join().unwrap();
    }
    #[test]
    fn test_mutex() {
        let counter = Arc::new(Mutex::new(0));
        let mut handler_ver = vec![];
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let hander = thread::spawn(move || match counter_clone.try_lock() {
                Ok(_) => println!("lock succeed"),
                Err(error) => println!("lock failed {}", error),
            });
            handler_ver.push(hander);
        }
        for h in handler_ver {
            h.join().unwrap()
        }
    }

    #[test]
    fn test_rw_mutex() {
        let counter = Arc::new(RwLock::new("hw".to_owned()));
        let mut handler_ver = vec![];
        for _ in 0..10 {
            {
                let counter_clone = Arc::clone(&counter);
                let hander = thread::spawn(move || match counter_clone.try_write() {
                    Ok(mut lock) => {
                        println!("wlock succeed");
                        lock.push_str("s");
                    },
                    Err(error) => println!("wlock failed {}", error),
                });
                handler_ver.push(hander);
            }

            {
                let counter_clone = Arc::clone(&counter);
                let hander = thread::spawn(move || match counter_clone.try_read() {
                    Ok(lock) => {
                        println!("rlock succeed {}",lock.as_str()); 
                    }
                    Err(error) => println!("rlock failed {}", error),
                });
                handler_ver.push(hander);
            }
        }
        for h in handler_ver { 
            h.join().unwrap()
        }
    }
}
