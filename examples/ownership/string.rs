fn main() {
    let mut s1 = String::from("Hello");
    s1.push_str(", world!");

    let s2 = s1;

    println!("{}", s2);
    // println!("{}", s1);
}
