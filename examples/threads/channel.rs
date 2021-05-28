use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let (tx1, rx) = mpsc::channel();

    let tx2 = tx1.clone();

    thread::spawn(move || {
        for i in 0..10 {
            thread::sleep(Duration::from_millis(50));
            let val = format!("hi {}", i);
            tx1.send(val).unwrap();
        }
    });
    thread::spawn(move || {
        for i in 0..10 {
            thread::sleep(Duration::from_millis(50));
            let val = format!("bonjour {}", i);
            tx2.send(val).unwrap();
        }
    });

    for msg in rx {
        println!("Got: {}", msg);
    }
}
