use std::ops::Mul;

fn square<T>(x: T) -> T
    where T: Copy + Mul<Output=T>
{
    return x * x;
}

fn main() {
    let x = 3 as i32;
    let y = 3.14 as f64;
    println!("hi!");
    println!("({})^2 = {}", y, square(y));
    println!("({})^2 = {}", x, square(x));
}
