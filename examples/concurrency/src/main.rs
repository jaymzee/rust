// concurrent programming on a single thread
// combine futures with join_all and they will run concurrently
//
use log::debug;
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};
use std::time::Duration;
use tokio::time;
use futures::future;

#[tokio::main]
async fn main() {
    // setup logging
    let config = ConfigBuilder::new()
        .set_target_level(LevelFilter::Trace)
        .build();
    let _ = SimpleLogger::init(LevelFilter::Debug, config);

    let futures: Vec<_> = (1..10).map(|x| foo(x)).collect();
    let result = future::join_all(futures).await;
    println!("result: {:?}", result);
}

async fn foo(i: i32) -> i32 {
    for j in 1..10 {
        debug!("in foo({}), iteration {}", i, j);
        time::sleep(Duration::from_millis(200)).await;
    }

    i
}
