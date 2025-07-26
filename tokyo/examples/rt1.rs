use tokio;

fn main() {
    // 创建Runtime
    // let rt = tokio::runtime::Runtime::new().unwrap();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8) // 这里的所说的线程是Rust线程，而每一个Rust线程都是一个OS线程。
        .enable_io() // io事件驱动
        .enable_time()  // 定时器
        .build()
        .unwrap();
    std::thread::sleep(
        std::time::Duration::from_secs(10)
    );

    rt.block_on(
        async {
            println!("Hello, world!");
        }
    )
    
}