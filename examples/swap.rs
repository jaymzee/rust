fn swap1() {
    let mut a = 2;
    let mut b = 3;
    std::mem::swap(&mut a, &mut b);
    println!("a = {a}, b = {b}", a=a, b=b);
}

fn swap2() {
    let a = 2;
    let b = 3;
    let (a, b) = (b, a);
    println!("a = {a}, b = {b}", a=a, b=b);
}

fn main() {
    swap1();
    swap2();
}
