use std::collections::HashSet;

fn main() {
    let mut hs: HashSet<i32> = HashSet::new();

    hs.insert(2);
    hs.insert(3);

    println!("{:?}", hs);
}
