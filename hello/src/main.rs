mod lib;
mod bin;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};
use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(10);

    for x in listener.incoming() {
        let stream = x.unwrap();
        println!("Connection established!");
        pool.execute(handle_connection(stream));


        // thread::spawn(|| {
        //     handle_connection(stream);
        // });
    }
}


fn handle_connection(mut stream: TcpStream) {
    let _buf_reader = BufReader::new(&mut stream);
    let _request: Vec<_> = _buf_reader
        .lines()
        .map(|r| r.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();

    // println!("request: \n {:#?}", request);

    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = fs::read_to_string("/Users/changyouliang/project/rustproject/corrosion/hello/src/hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");


    stream.write_all(response.as_bytes()).unwrap();
}