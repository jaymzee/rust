fn main() {
    let mut v = vec![1, 2, 3];
    v.push(5);

    for i in &v {
        println!("{}", i);
        // v.push(42);  // prevented by borrow
    }
    println!("v = {:?}", v);
}
