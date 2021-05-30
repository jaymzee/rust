fn f(x: i32) {
    println!("{}", x);
}

use std::thread;

fn main() {
    let threads: Vec<_> = (0..20)
        .map(|i| thread::spawn(move || f(i)))
        .collect();

    for t in threads {
        t.join()
            .expect("failed to join main thread");
    }
    println!("done");
}
