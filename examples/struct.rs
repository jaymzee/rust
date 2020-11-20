struct Foo {
    x: i32,
    y: i32
}

fn main() {
    let s = Foo{x: 2, y: 3};
    println!("{{ {}, {} }}", s.x, s.y);
}
