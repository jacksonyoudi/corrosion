use mini_redis::{client, Result};

#[tokio::main]
// async fn main() -> Result<()> {
async fn main_test() {
    // let mut result = client::connect("127.0.0.1:6379").await?;
    // result.set("hello", "word".into()).await?;
    //
    // let r = result.get("hello").await?;
    //
    //
    // println!("got value from the server; result={:?}", r);
    //
    // Ok(())

    let op = say_world();
    println!("hello");
    op.await;
}


async fn say_world() {
    println!("world")
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(
        async {
            println!("hello")
        }
    )
}
