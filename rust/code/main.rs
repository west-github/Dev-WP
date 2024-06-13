#![allow(dead_code, unused_variables)]
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

async fn my_task(id: u64, sem: Arc<Semaphore>) -> Result<()> {
    let _f = sem.acquire().await?; // Acquire a permit
    sleep(Duration::from_secs(id)).await;
    println!("Task {} is done", id);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let sem = Arc::new(Semaphore::new(10)); // Limit to 10 concurrent tasks
    let mut handles = vec![];

    for i in 0..100 {
        handles.push(tokio::spawn(my_task(i, sem.clone())));
    }

    for handle in handles {
        handle.await??;
    }

    println!("All tasks are done");

    Ok(())
}
