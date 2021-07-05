fn make_func(y: i32) -> impl Fn(i32) -> i32 {
    move |x| x * y
}

fn main() {
    let mul3 = make_func(3);
    let mul5 = make_func(5);

    println!("{}", mul5(2));
    println!("{}", mul3(2));
}
