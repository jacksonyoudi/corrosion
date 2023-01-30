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

pub fn channel_event_test() {
    let (work_sender, work_receiver) = channel();
    let (result_sender, result_receiver) = channel();

    thread::spawn(move || loop {
        match work_receiver.recv() {
            Ok(WorkMsg::Work(num)) => {
                let _ = result_sender.send(ResultMsg::Result(num));
            }
            Ok(WorkMsg::Exit) => {
                let _ = result_sender.send(ResultMsg::Exit);
                break;
            }
            _ => panic!("Error receiving a WorkMsg.")
        }
    });

    let _ = work_sender.send(WorkMsg::Work(1));
    let _ = work_sender.send(WorkMsg::Work(2));
    let _ = work_sender.send(WorkMsg::Exit);

    let mut counter: u8 = 0;
    loop {
        match result_receiver.recv() {
            Ok(ResultMsg::Result(num)) => {
                println!("num: {}", num);
                println!("counter: {}", counter);
                counter += 1;
            }
            Ok(ResultMsg::Exit) => {
                println!("exit");
                println!("counter: {}", counter);
                break;
            }
            _ => panic!("Error receiving a ResultMsg.")
        }
    }
}