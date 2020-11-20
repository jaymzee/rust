fn main() {
    // fn can't capture dynamic environment
    fn square(x: i32) -> i32 {
        x * x
    }

    for i in 1..5 {
        print!("{} ", square(i));
    }
    println!();
}
