#[tokio::main]
async fn main() {
    println!("fibonacci(10) = {}", fibonacci(10));
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}