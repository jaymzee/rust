fn main() {
    let b = true;
    let x: i32;

    // complicated initialization
    if b {
        x = 42;
    } else {
        x = 88;
    }
    println!("{}", x);
}
