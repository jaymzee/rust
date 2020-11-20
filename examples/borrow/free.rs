fn main() {
    let y: &i32;
    {
        let x = 5;
        y = &x;
        println!("{}", y);
    }
    // borrowed value doesn't live long enough
    // println!("{}", y);
}
