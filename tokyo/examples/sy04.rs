use tokio::sync::Barrier;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let barries = Arc::new(Barrier::new(3));
    let mut handles = Vec::with_capacity(10);

    for _ in 0..10 {
        let c = barries.clone();
        handles.push(tokio::spawn(async move {
            println!("before wait");

            // 这里设置屏障
            let result = c.wait().await;
            println!("after wait, result: {:?}", result);
            result
        }));
    }

    let mut num_leaders = 0;
    for handle in handles {
        let wait_result = handle.await.unwrap();
        if wait_result.is_leader() {
            num_leaders += 1;
        }
    }

    assert_eq!(num_leaders, 1);

}