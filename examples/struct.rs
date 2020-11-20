struct Foo {
    x: i32,
    y: i32
}

impl Foo {
    fn bar(&self) -> i32 {
        self.x
    }

    fn baz(&mut self) {
        self.x += 1;
    }
}

fn main() {
    let mut f = Foo{x: 2, y: 3};

    println!("{} {}", f.bar(), f.y);
    f.baz();
    println!("{} {}", f.bar(), f.y);
}
