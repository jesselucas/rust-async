// Use the tokio boilerplate to handle a future
use tokio::runtime::Runtime;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let future = hello_world();
    let rt = Runtime::new().unwrap();
    let handle = rt.handle();
    handle.spawn(future);
}
