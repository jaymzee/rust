use std::env;

fn main() {
    for (i, arg) in env::args().enumerate() {
        println!("{}: {}", i, arg);
    }
}
