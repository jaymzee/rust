use nalgebra::{Matrix3, Vector3};
use ndarray::{Array2, Array1};
use ndarray_linalg::Solve;

pub fn example1() {
    let a = Matrix3::new(8.0, 1.0, 6.0, 3.0, 5.0, 7.0, 4.0, 9.0, 2.0);
    let b = Vector3::new(1.0, 2.0, 3.0);
    println!("A =\n{:.4}", a);
    println!("b =\n{:.4}", b);

    let a_lu = a.lu();
    println!("L = {}", Matrix3::from(a_lu.l()));
    println!("U = {}", Matrix3::from(a_lu.u()));
    println!("P = {:?}", a_lu.p());

    let x = a_lu.solve(&b).unwrap();
    println!("x =\n{:.4}", x);

    println!("A x = {:.4}", a * x);
}

pub fn example2() {
    let a: Array2<f64> = ndarray_linalg::random((3, 3));
    let b: Array1<f64> = ndarray_linalg::random(3);
    println!("A =\n{:.4}", a);
    println!("b =\n{:.4}", b);

    let x = a.solve_into(b).unwrap();
    println!("x =\n{:.4}", x);

    println!("A x = {:.4}", a.dot(&x));
}
