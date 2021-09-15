use nalgebra::Matrix3;
use ndarray::{Array2, Array1, array};
use ndarray_linalg::{Solve, Inverse};

pub fn matrix_inverse_example1() {
    let m1 = Matrix3::new(2., 1., 1.,
                          3., 2., 1.,
                          2., 1., 2.);
    println!("m1 = {}", m1);
    if let Some(inv) = m1.try_inverse() {
        println!("The inverse of m1 is: {}", inv);
    } else {
        println!("m1 is not invertible");
    }
}

pub fn matrix_inverse_example2() {
    let m1 = array![[2., 1., 1.],
                    [3., 2., 1.],
                    [2., 1., 2.]];
    println!("{}", m1);
    let m2 = m1.inv();
}

pub fn solve() {
    let a: Array2<f64> = ndarray_linalg::random((3, 3));
    let b: Array1<f64> = ndarray_linalg::random(3);
    let x = a.solve_into(b).unwrap();

    println!("{:.4}", a);
    println!("{:.4}", x);
}
