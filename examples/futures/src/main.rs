use futures::MyFuture;

fn main() {
    let my_future = MyFuture::default();

    println!("Output: {}", futures::run(my_future));
}
