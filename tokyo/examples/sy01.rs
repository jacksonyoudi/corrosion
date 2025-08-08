use std::sync::Arc;
use tokio::{self, sync, runtime::Runtime, time::{self, Duration}};



// tokio::sync::Mutex其内部使用了标准库的互斥锁，
// 即std::sync::Mutex，而标准库的互斥锁是针对线程的，
// 因此，使用tokio的互斥锁时也会锁住整个线程。
// 此外，tokio::sync::Mutex是对标准库的Mutex的封装，性能相对要更差一些。
// 也因此，官方文档中建议，如非必须，应使用标准库的Mutex或性能更高的parking_lot提供的互斥锁，而不是tokio的Mutex。
fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let mutex = Arc::new(sync::Mutex::new(0));

        for i in 0..10 {
            let lock = Arc::clone(&mutex);
            tokio::spawn(async move {
                let mut data = lock.lock().await;
                *data += 1;
                println!("task: {}, data: {}", i, data);
            });
        }

        time::sleep(Duration::from_secs(1)).await;
    });
}

// 什么是跨await？每个await都代表一个异步任务，跨await即表示该异步任务中出现了至少一个子任务。
// 而每个异步任务都可能会被tokio内部偷到不同的线程上执行，
// 因此跨await时要求其父任务实现Send Trait，这是因为子任务中可能会引用父任务中的数据