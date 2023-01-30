use rayon;
use std::sync::mpsc::channel;
use std::thread;

enum WorkMsg {
    Work(u8),
    Exit,
}

enum ResultMsg {
    Result(u8),
    Exit,
}

pub fn done_thread_channel_test() {
    let (work_sender, work_receiver) = channel();
    let (result_sender, result_receiver) = channel();

    // 引入线程池，开两个工作线程
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();

    let _ = thread::spawn(move || loop {
        match work_receiver.recv() {
            Ok(WorkMsg::Work(num)) => {
                let sender = result_sender.clone();
                pool.spawn(move || {
                    let _ = sender.send(ResultMsg::Result(num));
                })
            }
            Ok(WorkMsg::Exit) => {
                let _ = result_sender.send(ResultMsg::Exit);
                break;
            }
            _ => panic!("Error receiving a WorkMsg.")
        }
    });

    let _ = work_sender.send(WorkMsg::Work(0));
    let _ = work_sender.send(WorkMsg::Work(1));
    let _ = work_sender.send(WorkMsg::Exit);


    loop {
        match result_receiver.recv() {
            Ok(ResultMsg::Result(num)) => {
                // 不能再断言顺序了
                println!("num: {}", num);
            }
            Ok(ResultMsg::Exit) => {
                println!("Exit");
                // 也不能断言在退出消息之前已经收到了结果
                break;
            }
            _ => panic!("Error receiving a ResultMsg."),
        }
    }
}