#[tokio::main]
async fn main() {
    let future = async {
        println!("Testing async, look in examples");
    };

    future.await;
}
