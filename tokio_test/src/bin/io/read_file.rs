use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

fn main1() {
    println!("Hello, world!");
    let _ = read_lines("a.txt");
    let _ = read_lines(String::from("a.txt"));
    let _ = read_lines(Path::new("x.txt"));
    let _ = read_lines(PathBuf::from("x.txt"));
}

// 总结：使用异步的api
// 打开文件的时候，异步
// 读取每一行，异步
// 涉及到io的，都采用异步

#[tokio::main]
async fn main() {
    // warm up
    let _ = count_lines_sync();

    let start = Instant::now();

    let count = count_lines_sync();

    let duration = start.elapsed();
    println!("count: {}, duration: {:?}", count, duration);

    // fake async
    // tokio::join!(
    //     fake_read_lines(),
    //     fake_read_lines(),
    //     ticker(),
    // );

    let now = Instant::now();
    let _ = tokio::join!(
        count_lines_async(), 
        count_lines_async(), 
        // ticker(),
    );
    let duration = now.elapsed();
    println!("async count: {}, duration: {:?}", count, duration);
}

fn count_lines_sync() -> i32 {
    let mut count = 0;
    if let Ok(lines) = read_lines("Cargo.lock") {
        lines.for_each(|line| {
            if let Ok(line) = line
                && !line.is_empty()
            {
                count += 1;
            }
        });
    }
    count
}

async fn count_lines_async() -> anyhow::Result<usize> {
    use tokio::fs::File;
    use tokio::io::AsyncBufReadExt;
    use tokio::io::BufReader;

    println!("count_lines_async start");
    let now = Instant::now();
    let mut count = 0;
    let file = File::open("Cargo.lock").await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        if !line.trim().is_empty() {
            count += 1;
        }
    }

    let duration = now.elapsed();
    println!("count: {}, duration: {:?}", count, duration);
    Ok(count)
}

fn read_lines<T>(filename: T) -> anyhow::Result<io::Lines<io::BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// 假的异步
async fn fake_read_lines() {
    let start = Instant::now();

    let mut count = 0;
    if let Ok(lines) = read_lines("Cargo.lock") {
        lines.for_each(|line| {
            if let Ok(line) = line
                && !line.is_empty()
            {
                count += 1;
            }
        });
    }

    let duration = start.elapsed();
    println!("fake async count: {}, duration: {:?}", count, duration);
}

async fn ticker() {
    for i in 0..10 {
        println!("ticker: {}", i);
        tokio::time::sleep(Duration::from_micros(1)).await;
    }
}
