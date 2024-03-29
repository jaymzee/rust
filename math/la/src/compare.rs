use ndarray::Array;
use approx::assert_abs_diff_eq;

pub fn compare() {
    let a = Array::from(vec![1., 2., 3., 4., 5.]);
    let b = Array::from(vec![5., 4., 3., 2., 1.]);
    let mut c = Array::from(vec![1., 2., 3., 4., 5.]);
    let mut d = Array::from(vec![5., 4., 3., 2., 1.]);

    let z = a + b;
    let w = &c + &d;

    assert_abs_diff_eq!(z, Array::from(vec![6., 6., 6., 6., 6.]));
}

