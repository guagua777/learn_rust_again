use async_recursion::async_recursion;

#[tokio::main]
async fn main() {
    println!("fibonacci(10) = {}", fibonacci(10).await);
}

#[async_recursion]
async fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1).await + fibonacci(n - 2).await,
    }
}
