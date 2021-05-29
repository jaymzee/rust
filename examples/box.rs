// Using Box<T> to point to data on the heap

use List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    let b = Box::new(List::Nil);
    let c = List::Cons(42, Box::new(List::Nil));
    let d = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", d);

    let e = MyBox::new(5);
    println!("{:?}", e);
    assert_eq!(5, *e);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
