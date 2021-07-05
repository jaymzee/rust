fn main() {
    fn f(x: i32) -> i32 {
        x * x
    }

    use_fn_or_closure(f);
    use_fn_or_closure(|x| 3 * x);
}

fn use_fn_or_closure(f: fn(i32) -> i32) {
    println!("{}", f(4));
}
