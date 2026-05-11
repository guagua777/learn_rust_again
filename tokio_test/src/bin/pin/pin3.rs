
#[tokio::main]
async fn main() {
    let future = async {
        println!("hello world");
    };

    // future.await;

    // 栈上的pin，而不是Box Pin
    tokio::pin!(future);
    (&mut future).await;
}