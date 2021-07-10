use std::boxed::Box;

fn main() {
    let one = List::cons(42, List::Nil);
    let two = List::cons(88, List::cons(42, List::Nil));

    println!("{:?}", one);
    println!("{:?}", two);
}

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> List<T> {
    fn cons(a: T, b: List<T>) -> List<T> {
        List::Cons(a, Box::new(b))
    }
}
