use futures::{MyFuture, AddOneFuture};

fn main() {
    let f1 = MyFuture::default();
    println!("Output: {}", futures::run(f1));
    let f2 = MyFuture::default();
    println!("Output: {}", futures::run(AddOneFuture(f2)));
}
