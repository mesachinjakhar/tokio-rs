use tokio::time::{Duration, timeout, interval};
use tokio::time::sleep;



#[tokio::main]
async fn main() {
    sleep(Duration::from_millis(100)).await;
    println!("100 ms have elapsed");

    let res = timeout(Duration::from_secs(1), long_future()).await;


    if res.is_err() {
        println!("operation timeout");
    }

    let mut interval = interval(Duration::from_secs(2));

    for _i in 0..5 {
    interval.tick().await;
    task_that_takes_a_second().await;
}
}

async fn long_future() {
    // do work here
}

async fn task_that_takes_a_second() {
    println!("hello");
    sleep(Duration::from_secs(1)).await
}

