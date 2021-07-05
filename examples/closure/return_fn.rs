fn make_fn(i: i32) -> fn(i32) -> i32 {
    fn f(x: i32) -> i32 { x * x }
    fn g(x: i32) -> i32 { 2 * x }

    match i % 2 {
        0 => f,
        _ => g,
    }
}

fn main() {
    for i in 0..=1 {
        let f = make_fn(i);
        println!("{}", f(3));
    }
}
