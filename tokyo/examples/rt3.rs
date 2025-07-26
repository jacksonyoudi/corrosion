use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;

fn main() {
    // 可手动创建线程，并在不同线程内创建互相独立的runtime。

    let t1 = thread::spawn(
        || {
            let rt = Runtime::new().unwrap();
            thread::sleep(Duration::from_secs(10));
        }
    );

    let t2 = thread::spawn(
        || {
            let rt = Runtime::new().unwrap();
            thread::sleep(Duration::from_secs(10));
        }
    );

    t1.join().unwrap();
    t2.join().unwrap();
}