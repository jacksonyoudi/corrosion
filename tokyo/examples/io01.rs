use tokio::io::AsyncBufReadExt;

#[tokio::main]
async fn main() {
    let file = tokio::fs::File::open("/tmp/a.log").await.unwrap();
    let mut buf_reader = tokio::io::BufReader::new(file);
    let mut buf = String::new();

    loop {
        match buf_reader.read_line(&mut buf).await {
            Err(_e) => panic!("read file error"),
            Ok(_n) => {
                // read_line()总是保留行尾换行符(如果有的话)，因此使用print!()而不是println!()
                print!("{}", buf);
                // read_line()总是将读取的内容追加到buf，因此每次读取完之后要清空buf
                buf.clear();
            }
        }
    }
}