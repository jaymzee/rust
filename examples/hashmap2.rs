use std::collections::HashMap;

fn main() {
    let hm: HashMap<&str, i32> = [
        ("one", 1),
        ("two", 2)
    ].iter().cloned().collect();

    println!("{:?}", hm);
    for (k, v) in &hm {
        println!("key = {key}, value = {val}", key=k, val=v);
    }
}
