use ndarray::{array, Array1, ArrayView1};

pub fn norms_example() {
    let x = array![1., 2., 3., 4., 5.];
    println!("{:.4}", normalize(x));
}

pub fn normalize(mut x: Array1<f64>) -> Array1<f64> {
    let norm = l2_norm(x.view());
    x.mapv_inplace(|e| e/norm);
    x
}

pub fn l1_norm(x: ArrayView1<f64>) -> f64 {
    x.fold(0., |acc, elem| acc + elem.abs())
}

pub fn l2_norm(x: ArrayView1<f64>) -> f64 {
    x.dot(&x).sqrt()
}
