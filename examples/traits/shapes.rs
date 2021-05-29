trait Shape {
    fn area(&self) -> f32;
}

struct Rectangle {
    w: f32,
    h: f32
}
impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.w * self.h
    }
}

struct Circle {
    r: f32
}
impl Shape for Circle {
    fn area(&self) -> f32 {
       std::f64::consts::PI as f32 * self.r * self.r
    }
}

fn total_area(list: &[&dyn Shape]) -> f32 {
    list.iter().map(|&s| s.area()).sum()
}

fn main() {
    let rect = Rectangle { w: 2.0, h: 3.0 };
    let circ = Circle { r: 5.0 };
    let mut shapes: Vec<&dyn Shape> = vec![];
    shapes.push(&circ);
    println!("circle area: {}", circ.area());
    println!("total area: {}", total_area(&shapes));
    shapes.push(&rect);
    println!("rectangle area: {}", rect.area());
    println!("total area: {}", total_area(&shapes));
}
