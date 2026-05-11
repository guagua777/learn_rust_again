use std::future::Future;
use std::pin::Pin;

#[tokio::main]
async fn main() {
    let task = get_task("db");
    let result = task.await;
    println!("Got result: {result}");
}

// async 
// 该函数不要加async
// 为什么？
fn get_task(source: &str) -> Pin<Box<dyn Future<Output = String>>> { // 使用trait object统一类型
    match source {
        "db" => Box::pin(from_db()),
        "api" => Box::pin(from_api()),
        _ => Box::pin(async { "Unknown source".to_string() }),
    }
}

async fn from_db() -> String {
    "DB data".to_string()
}

async fn from_api() -> String {
    "API data".to_string()
}