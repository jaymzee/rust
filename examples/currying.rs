fn add(a: i32, b: i32) -> i32 { a + b }

fn main() {
    let add5 = |x| add(5, x);
    println!("{}", add5(2));
}
