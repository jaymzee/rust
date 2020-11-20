fn main() {
    let x: Vec<i32> = (1 .. 10).collect();
    let y: Vec<i32> = x.iter().map(|&x| x * x).collect();

    println!("{:?}", y);
}
