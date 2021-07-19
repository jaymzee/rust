fn main() {
    println!("hi");

    for i in Counter::new(8).into_iter() {
        println!("{}", i);
    }
}

struct Counter {
    count: usize,
    last: usize
}

impl Counter {
    fn new(last: usize) -> Self {
        Counter { count: 0, last }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        match self.count {
            count if count < self.last => Some(count),
            _ => None
        }
    }
}

