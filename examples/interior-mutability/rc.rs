//use std::boxed::Box;    // exclusive ownership, mutable
//use std::rc::Rc;        // shared ownership, immutable
//use std::sync::Arc;     // shared ownership, immutable, thread safe
//use std::cell::Cell;    // interior mutability for owned copy types
use std::cell::RefCell; // interior mutability for owned non-copy types

// how Rc could be implemented to acheive interior mutability
//

fn main() {
    let joe = Student::new("joe".to_string(), 42);
    let s1 = Rc::new(joe);
    let s2 = s1.clone();
    println!("s1 = {:#?}", s1);
    println!("s2 = {:#?}", s2);
}

#[derive(Debug, Clone)]
struct Student {
    name: String,
    age: u32,
}

impl Student {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
}

#[derive(Debug)]
struct Rc<T: Clone> {
    inner: T,
    references: RefCell<usize>,
}

impl<T: Clone> Rc<T> {
    fn new(inner: T) -> Self {
        let references = RefCell::new(1);
        Rc { inner, references }
    }
}

impl<T: Clone> Clone for Rc<T> {
    fn clone(&self) -> Self {
        *self.references.borrow_mut() += 1;
        Self {
            inner: self.inner.clone(),
            references: self.references.clone()
        }
    }
}
