use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    // let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    // loop {
    //     let (socket, _) = listener.accept().await.unwrap();
    //     // 线程去执行
    //     tokio::spawn(
    //         async move {
    //             process(socket).await;
    //         }
    //     );
    // }

    // let handle = tokio::spawn(async {
    //     "return value"
    // });
    //
    // let out = handle.await.unwrap();
    // println!("GOT {}", out);



}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);
    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Got: {:?}", frame);

        // Respond with an error
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}