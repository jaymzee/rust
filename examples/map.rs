fn main() {
    let a = Some(3);
    let b: Result<_, String> = Ok(4);
    let c: Vec<i32> = (1..10).collect();
    let x = a.map(|x| x * x);
    let y = b.map(|x| x * x);
    let z: Vec<_> = c.iter().map(|x| x * x).collect();

    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
}
