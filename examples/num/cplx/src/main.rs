use std::f64::consts::PI;
use num::complex::Complex;

fn main() {
    let x = Complex::new(1.0, 2.0);
    let y = Complex::new(0.0, PI);

    println!("{} {:.4}", x, y.exp());
}
