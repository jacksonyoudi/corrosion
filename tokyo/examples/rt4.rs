use tokio::runtime::Runtime;
use tokio::time;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        tokio::spawn(
            async {
                time::sleep(time::Duration::from_secs(1)).await;
            }
        )
    });
}

fn async_task(rt: &Runtime) {
    rt.spawn(
        async {
            time::sleep(time::Duration::from_secs(1)).await;
        }
    );
}