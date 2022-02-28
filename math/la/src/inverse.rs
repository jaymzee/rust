use nalgebra::Matrix3;

pub fn nalgebra_example() {
    println!("matrix inverse using nalgebra Matrix3\n");
    let m1 = Matrix3::new(2., 1., 1.,
                          3., 2., 1.,
                          2., 1., 2.);
    println!("m1 = {}", m1);
    if let Some(inv) = m1.try_inverse() {
        println!("m1.inverse() = {}", inv);
    } else {
        println!("m1 is not invertible");
    }
}

use ndarray::array;
use ndarray_linalg::Inverse;

pub fn ndarray_example() {
    println!("matrix inverse using ndarray_linalg\n");
    let m1 = array![[2., 1., 1.],
                    [3., 2., 1.],
                    [2., 1., 2.]];
    println!("A =\n{}", m1);
    let m2 = m1.inv().unwrap();
    println!("inverse =\n{}", m2);
}
