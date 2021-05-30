use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];

    let th_handle = thread::spawn(move || {
        println!("here is a vector {:?}", v);
    });
    th_handle.join()
        .expect("failed to join main thread");
}
