fn foo(x: i32, maybe_y: Option<i32>) -> i32 {
    match maybe_y {
        Some(y) => x * y,
        None => x
    }
}

fn main() {
    println!("{}", foo(5, None));
    println!("{}", foo(5, Some(2)));
    println!("{}", foo(5, Some(3)));
}
