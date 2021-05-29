use std::collections::BTreeMap;

fn main() {
    let mut btm = BTreeMap::new();
    btm.insert("one", 1);
    btm.insert("two", 2);
    println!("{:?}", btm);
}
