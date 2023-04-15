use std::future::join;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn get_two_sites() {
    let handle_one: JoinHandle<()> = thread::spawn(|| download("http://www.foo.com".into()));
    let handle_two: JoinHandle<()> = thread::spawn(|| download("http://www.bar.com".into()));

    // 等待两个线程完成任务, 两个线程不会阻塞
    handle_one.join();
    handle_two.join();

    // main线程 等待上述两个线程都完成了, 才会退出
}

pub async fn get_two_site() {
    let future_one = async_download("https:://www.foo.com".into());
    let future_two = async_download("https:://www.bar.com".into());

    // 同时执行两个 future 使它们完成
    // join!(future_one, future_two)
}


fn download(url: String) {
    println!("{}", url);
    thread::sleep(Duration::from_secs(3));
}

async fn async_download(url: String) {
    println!("{}", url);
    thread::sleep(Duration::from_secs(3));
}

pub async fn test() {
    let site = get_two_site();
    site.await
}