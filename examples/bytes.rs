fn main() {
    let bytes = "Hello, world!".as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        println!("{:2} 0x{:02x} {}", i, b, b as char);
    }
}
