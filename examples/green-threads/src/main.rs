use log::debug;
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};
use std::time::Duration;
use tokio::{time, task};

#[tokio::main]
async fn main() {
    // setup logging
    let config = ConfigBuilder::new()
        .set_target_level(LevelFilter::Trace)
        .build();
    let _ = SimpleLogger::init(LevelFilter::Debug, config);

    // spawn 50 green threads (uses 4 OS threads on my machine)
    for i in 1..50 {
        task::spawn(
            async move {
                loop {
                    debug!("in spawn {}", i);
                    time::sleep(Duration::from_millis(200)).await;
                }
            }
        );
    }

    // wait a couple seconds before main thread terminates
    time::sleep(Duration::from_millis(2000)).await;
}

