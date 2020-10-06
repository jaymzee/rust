// demonstrate local variables

const Z: i32 = 42;

fn foo(name: &str) {
    println!("hello {}", name);
}

fn main() {
    let x: i8 = 100;
    let y: u8 = 5;
    let name = "george";
    let mut s = String::from("joe");

    foo(name);
    foo(&s);
    s.push_str(" shmo");
    foo(&s);
    println!("x = {}", Z + i32::from(x) + i32::from(y))
}
