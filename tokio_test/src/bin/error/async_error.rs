use anyhow::{Result};
use tokio::{ time::sleep};
use std::time::Duration;
use anyhow::Error;

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     fetch_user(3).await?;
//     Ok(())
// }



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut tasks = Vec::new();
    for i in 0..10 {
        tasks.push(fetch_user(i as u32));
    }
    let results = futures::future::join_all(tasks).await;
    println!("{:#?}", results);


    // 如果有一个失败了，整体就失败了
    // 不是 : Vec<String>
    let final_result: anyhow::Result<Vec<String>> = results.into_iter()
    .collect();
    let users = final_result?;
    println!("{:#?}", users);


    // let mut errors = Vec::new();

    // let users: Vec<_> = results
    // .into_iter()
    // .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
    // .collect();
    // println!("users{:#?}", users);
    // println!("errors{:#?}", errors);


    Ok(())
}

async fn fetch_user(id: u32) -> anyhow::Result<String> {
    if id == 3 {
        anyhow::bail!("User {id} does not exist");
    }

    sleep(Duration::from_secs(1)).await;
    Ok(format!("User_{id}"))
}