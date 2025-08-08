use std::sync::{Arc, MutexGuard};
use tokio::{self, runtime::Runtime, time::{self, Duration}, sync::Mutex};

async fn add_1(mutex: &Mutex<u64>) {
    let mut lock = mutex.lock().await;
    *lock += 1;

    // 子任务，跨await，且引用了父任务中的数据
    time::sleep(Duration::from_millis(*lock)).await;
}

// std::sync::MutexGuard未实现Send，因此父任务async move{}语句块是非Send的，于是编译报错。
// 但如果上面的示例中没有子任务sleep().await子任务，则编译无错，因为已经可以明确知道该Mutex所在的任务是在当前线程执行的。

fn main() {
    Runtime::new().unwrap().block_on(async {
        let mutex = Arc::new(Mutex::new(0));
        for _ in 0..10 {
            let lock = mutex.clone();
            tokio::spawn(async move {
                add_1(&lock).await;
            });
        }
    });
}