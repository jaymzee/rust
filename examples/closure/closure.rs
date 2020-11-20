fn main() {
    let y: i32 = 2;
    // closure captures dynamic environment
    let mult = |x: i32| -> i32 { x * y };

    for i in 1..5 {
        print!("{} ", mult(i));
    }
    println!();
}
