use std;
use chrono::Local;
use tokio::{self, runtime::Runtime, time};

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

fn main() {
    let rt1 = Runtime::new().unwrap();
    // 创建一个blocking thread，可立即执行(由操作系统调度系统决定何时执行)
    // 注意，不阻塞当前线程
    let task = rt1.spawn_blocking(|| {
        println!("in task: {}", now());
        // 注意，是线程的睡眠，不是tokio的睡眠，因此会阻塞整个线程
        std::thread::sleep(std::time::Duration::from_secs(10))
    });

    // 小睡1毫秒，让上面的blocking thread先运行起来
    std::thread::sleep(std::time::Duration::from_millis(1));
    println!("not blocking: {}", now());

    // 可在runtime内等待blocking_thread的完成
    rt1.block_on(async {
        rt1.spawn_blocking(|| std::thread::sleep(std::time::Duration::from_secs(10)));
        std::thread::spawn(|| std::thread::sleep(std::time::Duration::from_secs(10)));
        println!("after blocking task: {}", now());
        task.await.unwrap();
    });
}