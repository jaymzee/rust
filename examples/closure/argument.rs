fn main() {
    fn f(x: i32) -> i32 {
        x * x
    }

    use_fn_or_closure(f);

    let mut y = 5;

    use_closure2(|x| {
        y += 1;
        x * y
    });

    println!("{}", y);
}

// only works if closure does not capture
fn use_fn_or_closure(f: fn(i32) -> i32) {
    println!("{}", f(4));
}

fn use_closure2(mut f: impl FnMut(i32) -> i32) {
    println!("{}", f(3));
    println!("{}", f(4));
    println!("{}", f(5));
}
