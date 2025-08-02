use tokio::runtime::Runtime;
use tokio::sync;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let (tx, rx) = sync::oneshot::channel();
        tokio::spawn(
            async move {
                if tx.send(33).is_err() {
                    println!("send failed");
                }
            }
        );
        match rx.await {
            Ok(value) => println!("rec {:?}", value),
            Err(_) => println!("recv failed"),
        }

        //     let (tx, mut rx) = sync::oneshot::channel();
        //     loop {
        //         // 注意，select!中无需await，因为select!会自动轮询推进每一个分支的任务进度
        //         tokio::select! {
        //     _ = interval.tick() => println!("Another 100ms"),
        //     msg = &mut recv => {
        //         println!("Got message: {}", msg.unwrap());
        //         break;
        //     }
        // }
        //     }
    });


    //     另一个比较常见的使用场景是结合select!宏，此时应在recv前面加上&mut。例如：
    // 注意mut

}
