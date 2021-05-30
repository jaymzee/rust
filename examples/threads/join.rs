use std::thread;
use std::time::Duration;

fn main() {
    let th_handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("hello from spawned thread {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });
    println!("hello from main thread");
    thread::sleep(Duration::from_secs(3));
    println!("main thread work complete");
    th_handle.join()
        .expect("failed to join main thread");
    println!("joined");
}
