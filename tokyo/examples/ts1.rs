use std::thread;
use chrono::Local;
use tokio::runtime::Runtime;
use tokio::{task, time};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}
fn main() {
    let rt = Runtime::new().unwrap();
    let _guard = rt.enter();
    let handle = task::spawn(
        async {
            time::sleep(time::Duration::from_secs(1)).await;
            println!("hello, {}", now());
        }
    );
    handle.abort();
    thread::sleep(time::Duration::from_secs(4));
}