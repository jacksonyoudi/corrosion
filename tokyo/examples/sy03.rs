use std::sync::Arc;
use tokio::sync::Notify;


#[tokio::main]
async fn main() {
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    tokio::spawn(async move {
       notify2.notified().await;
        println!("received notification");
    });


    // 每当调用notify_one()时，将产生一个执行权，但多次调用也最多只有一个执行权。
    // 因此，调用notify_one()之后再调用notified().await则并无需等待。
    println!("sending notification");
    notify.notify_one();

}