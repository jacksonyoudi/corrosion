use std::sync::mpsc::channel;
use std::thread;

pub enum WorkMsg {
    Work(u8),
    Exit,
}

pub enum ResultMsg {
    Result(u8),
    Exit,
}



fn main() {
    // let (work_sender, work_receiver) = channel();
    // let (result_sender, result_receiver) = channel();
    //
    // // 添加一个新的Channel，Worker使用它来通知“并行”组件已经完成了一个工作单元
    // let (pool_result_sender, pool_result_receiver) = channel();
    //
    // let mut ongoing_work = 0;
    // let mut exiting = false;
    //
    // // 使用线程池
    // let pool = rayon::ThreadPoolBuilder::new()
    //     .num_threads(2)
    //     .build()
    //     .unwrap();
    //
    // let _ = thread::spawn(move || loop {
    //     // select! {
    //     //     recv(work_receiver) -> msg => {
    //     //         ma
    //     //     }
    //     // }
    // });

}
