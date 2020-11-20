fn foo(v: Vec<i32>) {
    println!("v = {:?}", v);
}

fn main() {
    let x = vec![1, 2, 3, 4];
    foo(x);
    // next line not allowed because x was moved
    // println!("x = {:?}", x);
}
