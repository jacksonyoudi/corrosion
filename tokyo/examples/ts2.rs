use tokio::runtime::Runtime;
use tokio::time;


async fn sleep(n: u64) -> u64 {
    time::sleep(time::Duration::from_secs(n)).await;
    n
}


fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(
        async {
            // 阻塞的
            tokio::select! {
                v = sleep(5) => println!("v: {}", v),
                v = sleep(3) => println!("v: {}", v),
            }
            println!("select! done")
        }
    );
}