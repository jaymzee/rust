#[derive(Debug)]
struct BinTree<T> {
    value: T,
    left: Option<Box<BinTree<T>>>,
    right: Option<Box<BinTree<T>>>,
}

fn main() {
    let bt = BinTree {
        value: 7,
        left: Some(Box::new(BinTree {
            value: 3,
            left: None,
            right: None
        })),
        right: Some(Box::new(BinTree {
            value: 9,
            left: None,
            right: None
        })),
    };
    println!("{:?}", bt);
}
