use nalgebra::Matrix3;

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

use ndarray::array;
use ndarray_linalg::Inverse;

pub fn matrix_inverse_example2() {
    let m1 = array![[2., 1., 1.],
                    [3., 2., 1.],
                    [2., 1., 2.]];
    println!("A =\n{}", m1);
    let m2 = m1.inv().unwrap();
    println!("inverse =\n{}", m2);
}
