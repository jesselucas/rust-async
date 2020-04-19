// https://docs.rs/tokio/0.2.18/tokio/runtime/struct.Runtime.html#examples-2
// Use the tokio runtime to handle a future
async fn hello_world() {
    println!("hello, world!");
}

#[tokio::main]
async fn main() {
    hello_world().await;
}
