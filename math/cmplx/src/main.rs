use std::f64::consts::PI;
use num::complex::Complex;

fn main() {
    let x = Complex::new(0.0, PI);
    println!("e^(πj) = {}", x.exp()); // = -1
}
