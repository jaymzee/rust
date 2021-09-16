struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }
}

fn main() {
    for i in Fibonacci::new().take(10) {
        println!("{}", i);
    }
    for i in Fibonacci::new().skip(10).take(3) {
        println!("{}", i);
    }
}
