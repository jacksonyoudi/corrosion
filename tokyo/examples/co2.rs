use tokio::{
    self,
    runtime::Runtime,
    sync,
};

fn main() {
    Runtime::new().unwrap().block_on(async {
        let (tx, mut rx) = sync::mpsc::channel(10);

        tokio::spawn(
            async move {
                for i in 0..10 {
                    if tx.send(i).await.is_err() {
                        println!("receiver closed");
                        return;
                    }
                }
            }
        );
        while let Some(i) = rx.recv().await {
            println!("received: {}", i);
        }
    });
}

