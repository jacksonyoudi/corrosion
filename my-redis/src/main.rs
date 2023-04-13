use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 创建一个链接 ?
    let mut cl = client::connect("127.0.0.1:6379").await?;
    // set value 隐式转换
    cl.set("hello", "world".into()).await?;

    



}
