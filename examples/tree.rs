#[derive(Debug)]
pub struct Node<T> {
    value: T,
    children: Vec<Node<T>>,
}

fn main() {
    let tree = Node {
        value: 42,
        children: vec![
            Node {
                value: 8,
                children: vec![],
            }
        ]
    };

    println!("{:?}", tree);
}
